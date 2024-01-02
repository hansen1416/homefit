<script>
	import { onDestroy, onMount } from "svelte";
	import ThreeScene from "../lib/ThreeScene";
	import Stats from "three/examples/jsm/libs/stats.module.js";
	import { loadFBX, invokeCamera } from "../utils/ropes";
	import PlayerController from "../lib/PlayerController";
	import PoseDetector from "../lib/PoseDetector";

	/** @type {HTMLVideoElement} */
	let video;
	/** @type {HTMLCanvasElement} */
	let canvas;

	let showVideo = false;
	let animationPointer = 0;

	let cameraInvoked = false;

	/** @type {ThreeScene} */
	let threeScene;

	let stats;

	/** @type {PoseDetector} */
	let poseDetector = new PoseDetector();
	/** @type {PlayerController} */
	let playerController = undefined;

	let capture_pose = false;

	function animate() {
		// update physics world and threejs renderer
		threeScene.onFrameUpdate(stats);

		if (capture_pose) {
			poseDetector.predict(video);
		}

		animationPointer = requestAnimationFrame(animate);
	}

	onMount(() => {
		const sceneWidth = document.documentElement.clientWidth;
		const sceneHeight = document.documentElement.clientHeight;

		threeScene = new ThreeScene(canvas, sceneWidth, sceneHeight);

		threeScene.scene.position.set(0, -100, 0);

		if (import.meta.env.DEV) {
			stats = new Stats();
			stats.showPanel(1);
			document.body.appendChild(stats.dom);
		}

		invokeCamera(video, () => {
			cameraInvoked = true;
		});

		Promise.all([
			loadFBX("fbx/mixamo2.fbx"),
			loadFBX("fbx/mixamo0.fbx"),
			poseDetector.init(poseCallback),
		]).then(([fbx, fbx0, _]) => {
			// console.log(fbx, fbx0);

			// threeScene.scene.add(fbx);

			playerController = new PlayerController(fbx0);

			threeScene.scene.add(fbx0);

			animate();
		});
	});

	/**
	 * Out of onMount, beforeUpdate, afterUpdate and onDestroy,
	 * this is the only one that runs inside a server-side component.
	 */
	onDestroy(() => {
		cancelAnimationFrame(animationPointer);
	});

	function poseCallback(keypoints3D) {
		playerController.applyPose2Bone(keypoints3D, true);
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
		style="position: absolute; top:0; left: 0; display: {showVideo
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
			{#if showVideo}
				<button
					on:click={() => {
						showVideo = !showVideo;
					}}>hide video</button
				>
			{:else}
				<button
					on:click={() => {
						showVideo = !showVideo;
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

	.controls {
		position: absolute;
		bottom: 0;
		right: 0;
		padding: 10px;
		display: flex;
		justify-content: space-between;
	}
</style>
