<script>
	import Scene from "../components/Scene.svelte";
	import { onDestroy, onMount } from "svelte";
	import { loadFBX } from "../utils/ropes";
	import { websocket, websocket_state } from "../store/websocketStore";
	import { watch } from "../store/watch.js";

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
	});

	onDestroy(() => {});

	$: watch(websocket_state, ($websocket_state) => {
		if ($websocket_state === WebSocket.OPEN) {
			wsClient.sendMessage("redis://greeting");
		}
	});
</script>

<Scene {diva} shadow={undefined} />
