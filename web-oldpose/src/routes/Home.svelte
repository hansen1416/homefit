<script>
	import _ from "lodash";
	import { onDestroy, onMount } from "svelte";
	import { areAllValuesTrue } from "../utils/ropes";
	import { getDiva } from "../utils/methods";
	import { websocket, websocket_state } from "../store/websocketStore";
	import animation_queue from "../store/timelineStore";
	import animation_data from "../store/animationDataStore";
	import Scene from "../components/Scene.svelte";
	import TextBubble from "../components/TextBubble.svelte";
	import Menu from "../components/Menu.svelte";

	let diva;
	let wsClient;

	let animation_request_sent = false;

	let text_bubble = "";

	let show_menu = false;

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

	// {
	// 	greeting: false,
	// 	"pointing-forward": false,
	// };
	const animation_status = Object.fromEntries(
		animation_required.map((animation) => [animation.name, false])
	);

	onMount(() => {
		// we need store to keep diva and shadow
		Promise.all([getDiva()]).then(([fbx0]) => {
			diva = fbx0;

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
			// todo render menu component
			show_menu = true;
		} else {
			// check is current animation item has a `message` field, if yes, render TextBubble component
			const current_animation = a_queue[0];
			if (current_animation.message) {
				// render TextBubble component
				text_bubble = current_animation.message;
			}
		}
	});
</script>

<Scene {diva} shadow={undefined} />
<!-- render TextBubble on text_bubble -->
{#if text_bubble}
	<TextBubble text={text_bubble} />
{/if}

{#if show_menu}
	<Menu />
{/if}
