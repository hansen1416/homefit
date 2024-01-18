<script>
	import _ from "lodash";
	import { onDestroy, onMount } from "svelte";

	export let id;

	onMount(() => {
		console.log(id);
	});
</script>
