<script>
	import _ from "lodash";
	import { onDestroy, onMount } from "svelte";
	import { loadDiva, loadScenery } from "../utils/mediaLoader";

	import { diva, scenery } from "../store/archetypeStore";

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

	onDestroy(() => {});
</script>
