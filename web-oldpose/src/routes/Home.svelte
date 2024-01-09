<script>
	import Scene from "../components/Scene.svelte";
	import { onDestroy, onMount } from "svelte";
	import { loadFBX } from "../utils/ropes";
	import { websocket, websocket_state } from "../store/websocketStore";
	import { watch } from "../store/watch.js";
	import animation_queue from "../store/timelineStore";


	let diva;
	let wsClient;


	onMount(() => {
		Promise.all([loadFBX("fbx/mixamo0.fbx")]).then(([fbx0]) => {
			diva = fbx0;

			wsClient = $websocket;

			wsClient.onMessage = (msg) => {
				console.log(msg);
			};
		});


		setInterval(() => {
			// add a random string
			// timeline.add(Math.random().toString(36).substring(7));
			$animation_queue = [Math.random().toString(36).substring(7)];
		}, 3000)
	});

	onDestroy(() => {});

	$: watch(websocket_state, ($websocket_state) => {
		if ($websocket_state === WebSocket.OPEN) {
			wsClient.sendMessage("redis://greeting");
		}
	});


</script>

<Scene {diva} shadow={undefined} />
