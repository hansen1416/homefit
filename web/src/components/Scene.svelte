<script>
	import { onDestroy, onMount } from "svelte";
	import * as THREE from "three";
	import ThreeScene from "../lib/ThreeScene";
	// import RapierWorld from "../lib/RapierWorld";
	import Stats from "three/examples/jsm/libs/stats.module.js";
	// import { FBXLoader } from "three/examples/jsm/loaders/FBXLoader";
	import {
		loadFBX,
		createPoseLandmarker,
		invokeCamera,
	} from "../utils/ropes";
	// import { OBJLoader } from "three/examples/jsm/loaders/OBJLoader.js";
	import { cloneDeep } from "lodash";
	import PlayerController from "../lib/PlayerController";

	/** @type {HTMLVideoElement} */
	let video;
	/** @type {HTMLCanvasElement} */
	let canvas;

	let showVideo = false;
	let animationPointer = 0;

	let invokedCamera = false;

	let capturePose = false;

	/** @type {ThreeScene} */
	let threeScene;
	// /** @type {RapierWorld} */
	// let physicsWorld;

	let stats;

	let mixer;

	const clock = new THREE.Clock();

	let model_ready = false;

	let poseDetector, poseDetectorAvailable;
	/** @type {PlayerController} */
	let playerController = undefined;

	function animate() {
		// update physics world and threejs renderer
		threeScene.onFrameUpdate(stats);

		if (model_ready) {
			mixer.update(clock.getDelta());
			// console.log(1);
		}

		// ========= captured pose logic
		if (
			capturePose &&
			video &&
			video.readyState >= 2 &&
			poseDetectorAvailable &&
			poseDetector
		) {
			poseDetectorAvailable = false;
			poseDetector.detectForVideo(
				video,
				performance.now(),
				onPoseCallback
			);
		}

		animationPointer = requestAnimationFrame(animate);
	}

	onMount(() => {
		const sceneWidth = document.documentElement.clientWidth;
		const sceneHeight = document.documentElement.clientHeight;

		threeScene = new ThreeScene(canvas, sceneWidth, sceneHeight);

		threeScene.scene.position.set(0, -100, 0);

		// Promise.all([import("@dimforge/rapier3d")]).then(([RAPIER]) => {
		// 	physicsWorld = new RapierWorld(RAPIER);
		// });

		if (import.meta.env.DEV) {
			stats = new Stats();
			stats.showPanel(1);
			document.body.appendChild(stats.dom);
		}

		createPoseLandmarker().then((pose) => {
			poseDetector = pose;

			poseDetectorAvailable = true;
		});

		Promise.all([
			loadFBX("mixamo2.fbx"),
			loadFBX("mixamo0.fbx"),
			// loadFBX("mixamo2.fbx"),
		]).then(([fbx, fbx0]) => {
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

	function onPoseCallback(result) {
		if (!result || !result.worldLandmarks || !result.worldLandmarks[0]) {
			poseDetectorAvailable = true;
			return;
		}

		const pose3D = cloneDeep(result.worldLandmarks[0]);
		// const pose2D = cloneDeep(result.landmarks[0]);

		if (playerController) {
			// set rotation to limbs
			playerController.applyPose2Bone(pose3D, true);
		}

		poseDetectorAvailable = true;
	}

	$: {
		// when clicked capture pose, if the camera is not invoked, invoke it
		// then the poseDetector will be available
		if (capturePose === true && invokedCamera === false) {
			invokedCamera = true;
			invokeCamera(video, () => {
				// theoritically, the poseDetector should be available now
				// todo output a message tells posedetector is functional
			});
		}
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
			{/if}

			<button
				class={capturePose ? "active" : ""}
				on:click={() => {
					capturePose = !capturePose;

					// console.log(big_obj);
				}}>Capture Pose</button
			>
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
