<script>
	import _ from "lodash";
	import { onDestroy, onMount } from "svelte";
	import { loadFBX, loadGLTF } from "../utils/ropes";
	import WebSocketClient from "../lib/WebSocketClient";
	import TextBubble from "../components/TextBubble.svelte";
	import Menu from "../components/Menu.svelte";

	import { derived } from "svelte/store";
	import { diva, scenery } from "../store/archetypeStore";
	import websocket_state from "../store/websocketStore";
	import animation_queue from "../store/animationQueueStore";
	import animation_data from "../store/animationDataStore";

	// websocket client
	let wsClient = new WebSocketClient();
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

	onMount(() => {
		// wsClient = $websocket;
		// we need store to keep diva and shadow
		Promise.all([
			loadFBX("fbx/taunt.fbx"),
			loadGLTF("glb/vr_exhibition_gallery_baked.glb"),
		])
			.then(([fbx, glb]) => {
				diva.set(fbx);

				const room = glb.scene;

				// scale room up by 20
				room.scale.set(40, 40, 40);
				// set room position to 10, 0, 0
				room.position.set(0, 0, -500);
				// rotate room 90 degree along z axis
				room.rotation.set(0, Math.PI / -2, 0);

				scenery.set(room);
			})
			.catch((err) => {
				console.error(err);
			});
	});

	let _derived_store = derived(
		[diva, websocket_state],
		([_diva, _websocket_state]) => {
			return [_diva, _websocket_state];
		},
	);

	const unsubscribe_derived_store = _derived_store.subscribe(
		([_diva, _websocket_state]) => {
			// when websocket is connected, and diva is loaded
			// request the animation data needed in this component from redis
			// make only send request once

			if (
				!_diva ||
				typeof _diva !== "object" ||
				_diva.isObject3D !== true
			) {
				// diva is not ready, do nothing
				return;
			}

			if (_websocket_state !== WebSocket.OPEN) {
				// websocket is not ready, do nothing
				return;
			}

			if (animation_request_sent) {
				return;
			}

			console.log(
				"diva and websocket ready, start send animation request",
			);

			const animation_list = [];

			for (let i = 0; i < animation_required.length; i++) {
				const animation = animation_required[i];
				animation_list.push(animation.name);
			}

			// when websocket is connected, request the animation data needed in this component
			wsClient.sendMessage("redis://" + animation_list.join(","));

			console.log(
				"request animation data from redis",
				"redis://" + animation_list.join(","),
			);

			animation_request_sent = true;
		},
	);

	const unsubscribe_animation_data = animation_data.subscribe((data) => {
		// check if all animation data is ready
		const animation_data_ready = animation_required.every((animation) => {
			return data[animation.name] ? true : false;
		});

		if (animation_data_ready) {
			console.log("animation data ready, send to animation_queue");
			// update animation_queue, it will trigger watch in Scene.svelte
			animation_queue.set(animation_required);
		}
	});

	const unsubscribe_animation_queue = animation_queue.subscribe((a_queue) => {
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

	onDestroy(() => {
		// unsubscribe all stores
		unsubscribe_derived_store();
		unsubscribe_animation_data();
		unsubscribe_animation_queue();
	});
</script>

<!-- render TextBubble on text_bubble -->
{#if text_bubble}
	<TextBubble text={text_bubble} />
{/if}

{#if show_menu}
	<Menu />
{/if}
