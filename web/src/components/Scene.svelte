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
	import * as THREE from "three";
	import ThreeScene from "../lib/ThreeScene";
	import RapierWorld from "../lib/RapierWorld";
	import Stats from "three/examples/jsm/libs/stats.module.js";
	import { browser } from "$app/environment"; // false on SSR, true in the browser
	// import { FBXLoader } from "three/examples/jsm/loaders/FBXLoader";
	import { loadJSON, loadFBX } from "../utils/ropes";
	// import { OBJLoader } from "three/examples/jsm/loaders/OBJLoader.js";

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

	let stats;

	let mixer;

	const clock = new THREE.Clock();

	let model_ready = false;

	function animate() {
		// update physics world and threejs renderer
		threeScene.onFrameUpdate(stats);

		if (model_ready) {
			mixer.update(clock.getDelta());
			// console.log(1);
		}

		// if (physicsWorld) {
		// 	physicsWorld.onFrameUpdate();
		// }

		animationPointer = requestAnimationFrame(animate);
	}

	onMount(() => {
		const sceneWidth = document.documentElement.clientWidth;
		const sceneHeight = document.documentElement.clientHeight;

		threeScene = new ThreeScene(canvas, sceneWidth, sceneHeight);

		threeScene.scene.position.set(0, -100, 0);

		Promise.all([import("@dimforge/rapier3d")]).then(([RAPIER]) => {
			physicsWorld = new RapierWorld(RAPIER);
		});

		if (import.meta.env.DEV) {
			stats = new Stats();
			stats.showPanel(1);
			document.body.appendChild(stats.dom);
		}

		Promise.all([
			loadFBX("mixamo1.fbx"),
			loadJSON("mixamo_anim.json"),
		]).then(([fbx, anim]) => {
			// console.log(fbx, anim);

			threeScene.scene.add(fbx);

			// // Create an AnimationMixer, and get the list of AnimationClip instances
			mixer = new THREE.AnimationMixer(fbx);
			// const clips = mesh.animations;

			// console.log(object.animations[0]);

			const clip = THREE.AnimationClip.parse(anim);

			console.log(clip);

			const action = mixer.clipAction(clip);

			action.play();

			model_ready = true;
		});

		// const fbxLoader = new FBXLoader();
		// fbxLoader.load(
		// 	// "standard_female.fbx",
		// 	// "chibi.fbx",
		// 	// "yoga1.fbx",
		// 	"Taunt.fbx",
		// 	// "mixamo1.fbx",
		// 	(object) => {
		// 		// object.traverse(function (child) {
		// 		//     if ((child as THREE.Mesh).isMesh) {
		// 		//         // (child as THREE.Mesh).material = material
		// 		//         if ((child as THREE.Mesh).material) {
		// 		//             ((child as THREE.Mesh).material as THREE.MeshBasicMaterial).transparent = false
		// 		//         }
		// 		//     }
		// 		// })
		// 		// object.scale.set(.01, .01, .01)

		// 		threeScene.scene.add(object);

		// 		// console.log(object);

		// 		// // Create an AnimationMixer, and get the list of AnimationClip instances
		// 		mixer = new THREE.AnimationMixer(object);
		// 		// const clips = mesh.animations;

		// 		console.log(object.animations[0]);

		// 		const action = mixer.clipAction(object.animations[0]);

		// 		action.play();

		// 		model_ready = true;
		// 	},
		// 	(xhr) => {
		// 		console.log((xhr.loaded / xhr.total) * 100 + "% loaded");
		// 	},
		// 	(error) => {
		// 		console.log(error);
		// 	}
		// );

		// fbxLoader.load(
		// 	"Taunt.fbx",
		// 	(object) => {
		// 		console.log("taunt", object);

		// 		threeScene.scene.add(object);

		// 		// console.log(object.animations[0].tracks);
		// 	},
		// 	(xhr) => {
		// 		console.log((xhr.loaded / xhr.total) * 100 + "% loaded");
		// 	},
		// 	(error) => {
		// 		console.log(error);
		// 	}
		// );

		// const objLoader = new OBJLoader();
		// objLoader.load("rpm.glb", (object) => {
		// 	threeScene.scene.add(object);

		// 	// console.log("glb obj", object);
		// });

		animate();
	});

	/**
	 * Out of onMount, beforeUpdate, afterUpdate and onDestroy,
	 * this is the only one that runs inside a server-side component.
	 */
	onDestroy(() => {
		if (browser) {
			cancelAnimationFrame(animationPointer);
		}
	});
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
		/* this will only affect <canvas> elements in this component */
		z-index: -1;
		position: absolute;
		top: 0;
		left: 0;
		bottom: 0;
		right: 0;
	}
</style>
