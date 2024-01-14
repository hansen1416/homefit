<script>
	import { onDestroy, onMount } from "svelte";
	import Movinwords from "movinwords";
	// import "movinwords/movinwords.css";
	import "../../node_modules/movinwords/dist/movinwords.css";

	let text = "Hello world!";

	onMount(() => {
		// Triggers start after 2 seconds.
		setInterval(() => {
			const mw = new Movinwords({
				el: ".text-bubble",
				// sentence: text,
				autostart: true,
				events: {
					start: (options) => {
						console.log("Started!", options);
					},
					wordTransitionStart: (options) => {
						console.log("Word Transition Started", options);
					},
					wordTransitionEnd: (options) => {
						console.log("Word Transition Ended", options);
					},
					end: (options) => {
						console.log("Ended!", options);

						text = "";
					},
				},
			});

			// mw.start();

			console.log(1111);
		}, 2000);
	});
</script>

<!-- <link
	rel="stylesheet"
	href="https://unpkg.com/movinwords/dist/movinwords.css"
/> -->
<div class="text-bubble">{text}</div>

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
