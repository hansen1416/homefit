<script>
	import _ from "lodash";
	import { onDestroy, onMount } from "svelte";
	import { loadDiva, loadScenery } from "../utils/mediaLoader";

	import { derived } from "svelte/store";
	import { diva, scenery } from "../store/archetypeStore";
	import websocket_state from "../store/websocketStore";
	import animation_queue from "../store/animationQueueStore";
	import animation_data from "../store/animationDataStore";
	import conversation from "../store/conversationStore";

	export let id;

	onMount(() => {
		Promise.all([loadDiva($diva), loadScenery($scenery)])
			.then(([fbx, room]) => {
				diva.set(fbx);

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

			// if (animation_request_sent) {
			// 	return;
			// }

			console.log(
				"diva and websocket ready, start send animation request",
			);

			// const animation_list = [];

			// for (let i = 0; i < animation_required.length; i++) {
			// 	const animation = animation_required[i];
			// 	animation_list.push(animation.name);
			// }

			// // when websocket is connected, request the animation data needed in this component
			// wsClient.sendMessage("redis://" + animation_list.join(","));

			// console.log(
			// 	"request animation data from redis",
			// 	"redis://" + animation_list.join(","),
			// );
		},
	);

	onDestroy(() => {
		unsubscribe_derived_store();
	});
</script>
