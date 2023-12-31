<script>
	import { onDestroy, onMount } from "svelte";
	// import * as THREE from "three";
	// @ts-ignore
	import { cloneDeep } from "lodash";
	import {
		createPoseLandmarker,
		loadGLTF,
		invokeCamera,
		readBuffer,
	} from "../utils/ropes";
	import ThreeScene from "../lib/ThreeScene";
	import RapierWorld from "../lib/RapierWorld";
	// import TerrainBuilder from "../lib/TerrainBuilder";
	// import ItemsManager from "../lib/ItemsManager";
	// import PlayerController from "../lib/PlayerController";
	import Api from "../request/Api";

	/**
	 * what do I need?
	 *
	 * class to build the scene, add objects
	 * 		this class will need ThreeScene and PhysicsWorld
	 * 		how does it handle moving object?
	 *
	 * class to map the joints to character rotations, only uppder body
	 * 		this class take model bones and pose3d as parameter
	 *
	 * class to record and watch a series of joints positions, only hands for now
	 * 		this class take pose3d as pamameter
	 * 		it determine toss and running
	 * 		how to pass the result to character and world?
	 *
	 * class to watch the pose, do running, defend and toss
	 * 		this class take model bones and pose3d as parameter
	 * 		how does it pass result to world?
	 *
	 *
	 * Factory Pattern:
	 * Use the factory pattern to create instances of game objects, such as items and characters.
	 * This will allow you to easily create new instances with different configurations without having to modify the constructor functions.
	 *
	 * Singleton Pattern:
	 * If you have any components that should only have a single instance throughout the game,
	 * such as a game manager or a resource manager,
	 * use the singleton pattern to ensure that only one instance is created.
	 *
	 * Observer Pattern:
	 * Use the observer pattern to handle user pose events and interactions between the player's model and the objects in the 3D scene.
	 * This pattern allows you to decouple the event source (user pose) from the event listeners (player's model and game objects),
	 * making it easier to add, remove,
	 * or modify event listeners without affecting the event source.
	 *
	 * use Observer for Architect
	 */

	/** @type {ThreeScene} */
	let threeScene;
	/** @type {RapierWorld} */
	let physicsWorld;


	let video, canvas;

	let cameraReady = false,
		assetReady = false;

	let poseDetector, poseDetectorAvailable;

	let runAnimation = false,
		showVideo = false,
		animationPointer;

	let api = new Api();

	const sceneWidth = document.documentElement.clientWidth;
	const sceneHeight = document.documentElement.clientHeight;

	onMount(() => {
		if (false) {
			invokeCamera(video, () => {
				cameraReady = true;
			});

			createPoseLandmarker().then((pose) => {
				poseDetector = pose;

				poseDetectorAvailable = true;
			});
		} else {
			cameraReady = true;
		}

		threeScene = new ThreeScene(canvas, sceneWidth, sceneHeight);
		/** @ts-ignore */
		// threeScene.camera.position.set(0, 2000, 2000);

		Promise.all([
			import("@dimforge/rapier3d"),
		]).then(
			([
				RAPIER,
			]) => {
				physicsWorld = new RapierWorld(RAPIER);


				
			}
		);
	});

	onDestroy(() => {
		cancelAnimationFrame(animationPointer);
	});

	// when mannequin, model and camera are erady, start animation loop
	$: if (cameraReady && assetReady) {
		animate();
		// set player pos, camera pos, control target
		// we need the animation to update at least once to let raycasting work
		// playerController.initPLayerPos({ x: 0, z: 0 });
	}

	function animate() {


		// update physics world and threejs renderer
		physicsWorld.onFrameUpdate();
		threeScene.onFrameUpdate();

		// ========= captured pose logic
		if (
			runAnimation &&
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

	function onPoseCallback(result) {
		if (!result || !result.worldLandmarks || !result.worldLandmarks[0]) {
			poseDetectorAvailable = true;
			return;
		}

		const pose3D = cloneDeep(result.worldLandmarks[0]);
		const pose2D = cloneDeep(result.landmarks[0]);

		poseDetectorAvailable = true;
	}
</script>

<div class="bg">
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

	<canvas bind:this={canvas} />

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

			{#if runAnimation}
				<button
					on:click={() => {
						runAnimation = !runAnimation;

						// console.log(big_obj);
					}}>Pause</button
				>
			{:else}
				<button
					on:click={() => {
						runAnimation = !runAnimation;
					}}>Run</button
				>
			{/if}
		</div>
	</div>
</div>

<style>
	.bg {
		background-color: #005C97;
	}

	.controls {
		position: absolute;
		bottom: 10px;
		right: 10px;
	}

	.controls button {
		padding: 2px 4px;
		font-size: 18px;
		text-transform: capitalize;
	}
</style>
