<script>
	import _ from "lodash";
	import { onDestroy, onMount } from "svelte";
	import { areAllValuesTrue, loadFBX, loadGLTF } from "../utils/ropes";
	import { websocket, websocket_state } from "../store/websocketStore";
	import { diva, scenery } from "../store/archetypeStore";
	import animation_queue from "../store/timelineStore";
	import animation_data from "../store/animationDataStore";
	import TextBubble from "../components/TextBubble.svelte";
	import Menu from "../components/Menu.svelte";
	// websocket client
	let wsClient;
	// make sure animation data only send once dispite of websocket state change
	let animation_request_sent = false;
	// conversation text from diva
	let text_bubble = "";
	// show menu when animation queue is empty
	let show_menu = false;
	// make sure menu only show when animation played, not when page first loaded
	let animation_played = false;

	const animation_required = [
		{
			name: "greeting",
			repeat: 1,
			message: "Hello, I am Anya, how may I assisst you today?",
		},
		{
			name: "pointing-forward",
			repeat: 1,
			message: "Here are available workout options.",
		},
	];

	// { animation_name: false,... };
	const animation_status = Object.fromEntries(
		animation_required.map((animation) => [animation.name, false]),
	);

	onMount(() => {
		loadGLTF("glb/vr_exhibition_gallery_baked.glb").then((gltf) => {
			gltf.scene.name = "scene";

			threeScene.scene.add(gltf.scene);

			threeScene.scene.getObjectByName("scene").scale.set(20, 20, 20);
		});

		// we need store to keep diva and shadow
		Promise.all([
			loadFBX("fbx/taunt.fbx"),
			loadGLTF("glb/vr_exhibition_gallery_baked.glb"),
		])
			.then(([fbx0, room]) => {
				diva.set(fbx0);

				scenery.set(room);

				wsClient = $websocket;

				wsClient.onMessage = (msg) => {
					// get animation data from redis
					// where in format like name::data
					// update animation_data
					if (typeof msg !== "string") {
						return;
					}

					// first split the message
					let [name, data] = msg.split("::");

					console.log("received animation data for " + name);

					animation_data[name] = data;

					animation_status[name] = true;

					if (areAllValuesTrue(animation_status)) {
						// update animation_queue, it will trigger watch in Scene.svelte
						animation_queue.set(animation_required);
					}
				};
			})
			.catch((err) => {
				console.error(err);
			});
	});

	onDestroy(() => {});

	// when websocket is connected, request the animation data needed in this component from redis
	websocket_state.subscribe((state_value) => {
		if (state_value === WebSocket.OPEN) {
			if (animation_request_sent) {
				return;
			}

			const animation_list = [];

			for (let i = 0; i < animation_required.length; i++) {
				const animation = animation_required[i];
				animation_list.push(animation.name);
			}

			// when websocket is connected, request the animation data needed in this component
			wsClient.sendMessage("redis://" + animation_list.join(","));

			animation_request_sent = true;
		}
	});

	animation_queue.subscribe((a_queue) => {
		if (a_queue.length === 0) {
			if (animation_played) {
				// when there is animation palyed, and the queue empty render menu component
				show_menu = true;
			}
		} else {
			animation_played = true;
			// check is current animation item has a `message` field, if yes, render TextBubble component
			const current_animation = a_queue[0];
			if (current_animation.message) {
				// render TextBubble component
				text_bubble = current_animation.message;
			}
		}
	});
</script>

<!-- render TextBubble on text_bubble -->
{#if text_bubble}
	<TextBubble text={text_bubble} />
{/if}

{#if show_menu}
	<Menu />
{/if}
