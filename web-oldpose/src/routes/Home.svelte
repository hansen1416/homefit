<script>
	import Scene from "../components/Scene.svelte";
	import { onDestroy, onMount } from "svelte";
	import { loadFBX, areAllValuesTrue } from "../utils/ropes";
	import { websocket, websocket_state } from "../store/websocketStore";
	import { watch } from "../store/watch.js";
	import animation_queue from "../store/timelineStore";
	import animation_data from "../store/animationDataStore";
	import _ from "lodash";

	let diva;
	let wsClient;

	const animation_required = [
		{ name: "greeting", repeat: 1 },
		{ name: "grumpy", repeat: 1 },
	];

	const animation_status = {
		greeting: false,
		grumpy: false,
	};

	onMount(() => {
		Promise.all([loadFBX("fbx/mixamo0.fbx")]).then(([fbx0]) => {
			diva = fbx0;

			wsClient = $websocket;

			wsClient.onMessage = (msg) => {
				// get animation data from redis
				// where in format like name::data
				// update animation_data

				// first split the message
				let [name, data] = msg.split("::");

				animation_data[name] = data;

				animation_status[name] = true;

				if (areAllValuesTrue(animation_status)) {
					// update animation_queue, it will trigger watch in Scene.svelte
					$animation_queue = animation_required;
				}
			};
		});
	});

	onDestroy(() => {});

	$: watch(websocket_state, ($websocket_state) => {
		if ($websocket_state === WebSocket.OPEN) {
			let animation_list = "";

			for (let i = 0; i < animation_required.length; i++) {
				const animation = animation_required[i];
				animation_list += animation.name + ",";
			}

			// when websocket is connected, request the animation data needed in this component
			wsClient.sendMessage("redis://" + animation_list);
		}
	});
</script>

<Scene {diva} shadow={undefined} />
