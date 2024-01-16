<script>
	import { onDestroy, onMount } from "svelte";
	import Typed from "typed.js";

	export let text;

	let typedInstance;

	onMount(() => {
		typedInstance = new Typed("#typed-text", {
			strings: [text],
			//   typeSpeed: 50,
			//   loop: false,
		});
	});

	onDestroy(() => {
		typedInstance.destroy();
	});

	$: if (typedInstance && text) {
		// each time text changes, destroy the old instance and create a new one
		typedInstance.destroy();

		typedInstance.strings = [text];
		typedInstance.start();
	}
</script>

<div id="text_bubble" class="text-bubble"></div>

<style>
	.text-bubble {
		position: absolute;
		top: 30px;
		left: 50%;
		width: 300px;
		min-height: 30px;
		margin-left: -150px;
		border: 1px solid #fff;
	}
</style>
