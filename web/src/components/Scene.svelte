<script>
	/**
	 * 
	 
		// logic goes here

		export let title;
		export let person;

		// this will update `document.title` whenever
		// the `title` prop changes
		$: document.title = title;

		$: {
			console.log(`multiple statements can be combined`);
			console.log(`the current title is ${title}`);
		}

		// this will update `name` when 'person' changes
		$: ({ name } = person);

		// don't do this. it will run before the previous line
		let name2 = name;

		// Only values which directly appear within the $: block will become dependencies
		// of the reactive statement.
		// For example, in the code below total will only update when x changes, but not y.
		let x = 0;
		let y = 0;

		
		function yPlusAValue(value) {
			return value + y;
		}

		$: total = yPlusAValue(x);

		// It is important to note that the reactive blocks are ordered
		// via simple static analysis at compile time,
		// and all the compiler looks at are the variables that are assigned to
		// and used within the block itself, not in any functions called by them.
		// This means that yDependent will not be updated
		// when x is updated in the following example:
		let x = 0;
		let y = 0;

		
		function setY(value) {
			y = value;
		}

		$: yDependent = y;
		$: setY(x);

		// Moving the line $: yDependent = y below $: setY(x)
		// will cause yDependent to be updated when x is updated.

	*/

	import { onDestroy, onMount } from "svelte";
	import ThreeScene from "../lib/ThreeScene";
	import RapierWorld from "../lib/RapierWorld";

	/** @type {HTMLVideoElement} */
	let video;
	/** @type {HTMLCanvasElement} */
	let canvas;

	let showVideo = false;
	let animationPointer = 0;

	/** @type {ThreeScene} */
	let threeScene;
	/** @type {RapierWorld} */
	let physicsWorld;

	function animate() {
		// update physics world and threejs renderer
		// threeScene.onFrameUpdate();
		// physicsWorld.onFrameUpdate();

		animationPointer = requestAnimationFrame(animate);
	}

	onMount(() => {
		// 		let stats;
		// if (import.meta.env.DEV) {
		// import Stats from "three/examples/jsm/libs/stats.module.js";
		// 	stats = new Stats();
		// 	stats.showPanel(1);
		// 	document.body.appendChild(stats.dom);
		// }

		const sceneWidth = document.documentElement.clientWidth;
		const sceneHeight = document.documentElement.clientHeight;

		threeScene = new ThreeScene(canvas, sceneWidth, sceneHeight);

		Promise.all([import("@dimforge/rapier3d")]).then(([RAPIER]) => {
			physicsWorld = new RapierWorld(RAPIER);
		});

		animate();
	});

	// onDestroy(() => {
	// 	cancelAnimationFrame(animationPointer);
	// });
</script>

<canvas bind:this={canvas} />
<canvas />

<video
	bind:this={video}
	autoPlay={true}
	width={480 / 2}
	height={360 / 2}
	style="position: absolute; top:0; left: 0; display: {showVideo
		? 'block'
		: 'none'}"
>
	<track label="English" kind="captions" default />
</video>

<style>
	canvas {
		/* this will only affect <p> elements in this component */
		z-index: -1;
		position: absolute;
		top: 0;
		left: 0;
		bottom: 0;
		right: 0;
	}
</style>
