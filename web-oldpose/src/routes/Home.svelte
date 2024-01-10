<script>
	import Scene from "../components/Scene.svelte";
	import { onDestroy, onMount } from "svelte";
	import { loadFBX } from "../utils/ropes";
	import { websocket, websocket_state } from "../store/websocketStore";
	import { watch } from "../store/watch.js";
	import animation_queue from "../store/timelineStore";
	import animation_data from "../store/animationDataStore";
	import _ from "lodash";

	let diva;
	let wsClient;

	onMount(() => {
		Promise.all([loadFBX("fbx/mixamo0.fbx")]).then(([fbx0]) => {
			diva = fbx0;

			wsClient = $websocket;

			wsClient.onMessage = (msg) => {
				console.log(msg);

				// get animation data from redis
				// where in format like name::data
				// update animation_data

				// first split the message
				let [name, data] = msg.split("::");
				animation_data[name] = data;

				queue = _.cloneDeep($animation_queue);

				queue.push(name);

				// update animation_queue
				$animation_queue = queue;
			};
		});
	});

	onDestroy(() => {});

	$: watch(websocket_state, ($websocket_state) => {
		if ($websocket_state === WebSocket.OPEN) {
			// when websocket is connected, request the animation data needed in this component
			wsClient.sendMessage("redis://greeting,grumpy");
		}
	});
</script>

<Scene {diva} shadow={undefined} />
