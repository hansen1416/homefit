import * as THREE from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls";
import { FBXLoader } from "three/examples/jsm/loaders/FBXLoader";

export const SceneProperties = {
	camera_height: 0,
	camera_far_z: 200,
};

Object.freeze(SceneProperties);

let instance;

export default class ThreeScene {
	/**
	 *
	 * @param {HTMLCanvasElement} canvas
	 * @param {number} width
	 * @param {number} height
	 * @returns
	 */
	constructor(canvas, width, height) {
		// make it a singleton, so we only have 1 threejs scene
		if (instance) {
			return instance;
		} else {
			instance = this;
		}

		this.scene = new THREE.Scene();

		this.scene.add(new THREE.AxesHelper(5));

		// this.camera = new THREE.OrthographicCamera(
		// 	width / -2, // left
		// 	width / 2, // right
		// 	height / 2, // top
		// 	height / -2, // bottom
		// 	0.1, // near
		// 	width * 2 // far
		// );

		// this.camera.zoom = 60; // zoom in by 50%
		// this.camera.position.set(0, 0.1, -4);

		this.camera = new THREE.PerspectiveCamera(
			75,
			width / height,
			0.01,
			4000
		);

		this.camera.position.set(
			0,
			SceneProperties.camera_height,
			-SceneProperties.camera_far_z
		);

		this.camera.updateProjectionMatrix(); // update the camera's projection matrix

		// env light
		this.scene.add(new THREE.AmbientLight(0xffffff, 0.1));

		/**
		// mimic the sun light. maybe update light position later
		this.light = new THREE.PointLight(0xffffff, 0.7);
		this.light.position.set(0, 100, 0);
		this.light.castShadow = true;
		// this.light.shadow.mapSize.width = 2048;
		// this.light.shadow.mapSize.height = 2048;
 */
		this.light = new THREE.DirectionalLight(0xffffff, 0.9);
		this.light.position.set(0, 1000, 0);
		this.light.castShadow = true;

		this.light.target = new THREE.Object3D();
		this.light.target.position.set(0, 0, 1000);

		this.scene.add(this.light);
		this.scene.add(this.light.target);

		// env fog
		// this.scene.fog = new THREE.Fog(0x000000, 50, 200);

		this.renderer = new THREE.WebGLRenderer({
			canvas: canvas,
			alpha: true,
			antialias: true,
		});

		this.renderer.shadowMap.enabled = true;
		// this.renderer.shadowMap.type = THREE.BasicShadowMap;
		this.renderer.shadowMap.type = THREE.PCFSoftShadowMap;
		this.renderer.toneMappingExposure = 0.5;

		this.controls = new OrbitControls(this.camera, canvas);

		this.renderer.setSize(width, height);
	}

	onFrameUpdate(stats) {
		this.controls.update();

		this.renderer.render(this.scene, this.camera);

		if (stats) {
			stats.update();
		}
	}

	resetControl() {
		this.controls.reset();
	}

	loadFbx(url) {
		const fbxLoader = new FBXLoader();
		fbxLoader.load(
			url,
			(object) => {
				// object.traverse(function (child) {
				//     if ((child as THREE.Mesh).isMesh) {
				//         // (child as THREE.Mesh).material = material
				//         if ((child as THREE.Mesh).material) {
				//             ((child as THREE.Mesh).material as THREE.MeshBasicMaterial).transparent = false
				//         }
				//     }
				// })
				// object.scale.set(.01, .01, .01)

				this.scene.add(object);

				console.log(object);

				// Create an AnimationMixer, and get the list of AnimationClip instances
				const mixer = new THREE.AnimationMixer(object);
				// const clips = mesh.animations;

				mixer.clipAction(object.animations[0]);
			},
			(xhr) => {
				console.log((xhr.loaded / xhr.total) * 100 + "% loaded");
			},
			(error) => {
				console.log(error);
			}
		);
	}

	/**
	unload(target:THREE.Object3D){
        target.removeFromParent();
        target.traverse((child:any) => {
            // disposing materials
            if (child.material && !child.material._isDisposed){
                // disposing textures
                for (const value of Object.values(child.material) as any[]){
                    if (!value) continue;
                    if (value.dispose && !value._isDisposed){
                        value.dispose();
                        value._isDisposed = true;
                    }
                }
                child.material.dispose();
                child.material._isDisposed = true;
            }
            // disposing geometries
            if (child.geometry?.dispose && !child.geometry._isDisposed){
                child.geometry.dispose();
                child.geometry._isDisposed = true;
            }
        });
    }

	missing child.skeleton.boneTexture.dispose(); and you all set :+1:
but if you never use skinned mesh, you can skip this.
 */
}
