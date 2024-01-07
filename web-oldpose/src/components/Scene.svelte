<script>
	import { onDestroy, onMount } from "svelte";
	import ThreeScene from "../lib/ThreeScene";
	import Stats from "three/examples/jsm/libs/stats.module.js";
	import { invokeCamera } from "../utils/ropes";
	import PlayerController from "../lib/PlayerController";
	import PoseDetector from "../lib/PoseDetector";

	/** @type {HTMLVideoElement} */
	let video;
	/** @type {HTMLCanvasElement} */
	let canvas;

	/** @type {ThreeScene} */
	let threeScene;
	/** @type {Stats} */
	let stats;

	/** @type {PoseDetector} */
	let poseDetector = new PoseDetector();
	/** @type {PlayerController} */
	let playerController = undefined;

	let capture_pose = false;
	let detector_ready = false;
	let show_video = false;
	let animation_pointer = 0;

	// diva is the main character, play standard animation
	export let diva;
	// shaow is user pose projection
	export let shadow;

	function animate() {
		// update physics world and threejs renderer
		threeScene.onFrameUpdate(stats);

		if (detector_ready && capture_pose) {
			// todo try to run prediction asynchrously
			poseDetector.predict(video);
		}

		animation_pointer = requestAnimationFrame(animate);
	}

	onMount(() => {
		threeScene = new ThreeScene(
			canvas,
			document.documentElement.clientWidth,
			document.documentElement.clientHeight
		);
		// -100 is ground level
		threeScene.scene.position.set(0, -100, 0);

		if (import.meta.env.DEV) {
			stats = new Stats();
			stats.showPanel(1);
			document.body.appendChild(stats.dom);
		}
		// initialize camera
		invokeCamera(video, () => {});
		// initialize pose detector
		Promise.all([poseDetector.init(poseCallback)]).then(([_]) => {
			detector_ready = true;
		});

		animate();
	});

	/**
	 * Out of onMount, beforeUpdate, afterUpdate and onDestroy,
	 * this is the only one that runs inside a server-side component.
	 */
	onDestroy(() => {
		cancelAnimationFrame(animation_pointer);
	});

	/**
	 *
	 * @param {Array<{x: number, y: number, z: number, visibility: number}>} keypoints3D
	 */
	function poseCallback(keypoints3D) {
		if (playerController) {
			playerController.applyPose2Bone(keypoints3D, true);
		}
	}

	$: if (typeof diva === "object" && diva.isObject3D === true) {
		threeScene.scene.add(diva);
	}

	$: if (typeof shadow === "object" && shadow.isObject3D === true) {
		playerController = new PlayerController(shadow);

		threeScene.scene.add(shadow);
	}
</script>

<!-- section is not needed, only for readablity -->
<section>
	<canvas bind:this={canvas} />

	<video
		bind:this={video}
		autoPlay={true}
		width={480 / 2}
		height={360 / 2}
		style="position: absolute; top:0; left: 0; display: {show_video
			? 'block'
			: 'none'}"
	>
		<track label="English" kind="captions" default />
	</video>

	<div class="controls">
		<div>
			<button
				on:click={() => {
					threeScene.resetControl();
				}}>Reset Control</button
			>
			<!-- 
			{#if show_video}
				<button
					on:click={() => {
						show_video = !show_video;
					}}>hide video</button
				>
			{:else}
				<button
					on:click={() => {
						show_video = !show_video;
					}}>show video</button
				>
			{/if} -->

			<button
				class={capture_pose ? "active" : ""}
				on:click={() => {
					capture_pose = !capture_pose;
				}}><img src="svg/camera.svg" alt="Play" /></button
			>

			<button on:click={() => {}}>
				<img src="svg/play.svg" alt="Camera" />
			</button>
		</div>
	</div>
</section>

<style>
	canvas {
		/* this will only affect <canvas> elements in this component */
		z-index: -1;
		position: absolute;
		top: 0;
		left: 0;
		bottom: 0;
		right: 0;
	}
</style>
