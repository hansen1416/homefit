<script>
	import Scene from "../components/Scene.svelte";
	import { onDestroy, onMount } from "svelte";
	import { loadFBX } from "../utils/ropes";

	let diva;

	onMount(() => {
		Promise.all([loadFBX("fbx/mixamo0.fbx")]).then(([fbx0]) => {
			diva = fbx0;
		});
	});

	onDestroy(() => {});
</script>

<Scene {diva} shadow={undefined} />
