/**
 * @typedef {import('../../node_modules/@dimforge/rapier3d/pipeline/world').World} World
 * @typedef {import('../../node_modules/@dimforge/rapier3d/geometry/collider').Collider} Collider
 * @typedef {import('../../node_modules/@dimforge/rapier3d/geometry/collider').ColliderDesc} ColliderDesc
 * @typedef {import('../../node_modules/@dimforge/rapier3d/dynamics/rigid_body').RigidBody} RigidBody
 * @typedef {import('../../node_modules/@dimforge/rapier3d/dynamics/rigid_body').RigidBodyDesc} RigidBodyDesc
 * @typedef {import('../../node_modules/@dimforge/rapier3d/dynamics/coefficient_combine_rule').CoefficientCombineRule} CoefficientCombineRule
 * @typedef {import('../../node_modules/@dimforge/rapier3d/control/character_controller').KinematicCharacterController} KinematicCharacterController
 * @typedef {import('../../node_modules/@dimforge/rapier3d/geometry/ray').Ray} Ray
 * @typedef {import('../../node_modules/@dimforge/rapier3d/pipeline/query_pipeline').QueryFilterFlags} QueryFilterFlags
 * @typedef {{x: number, y: number, z: number}} vec3
 */

import * as THREE from "three";

let instance;

export default class RapierWorld {
	/**
	 *  Larger values of the damping coefficients lead to a stronger slow-downs. Their default values are 0.0 (no damping at all).
	 */
	liner_damping = 0.5;

	/**
	 * A friction coefficient of 0 implies no friction at all (completely sliding contact)
	 * and a coefficient greater or equal to 1 implies a very strong friction. Values greater than 1 are allowed.
	 */
	friction = 0.5;

	/**
	 * A restitution coefficient set to 1 (fully elastic contact) implies that
	 * the exit velocity at a contact has the same magnitude as the entry velocity along the contact normal:
	 * it is as if you drop a bouncing ball and it gets back to the same height after the bounce.
	 *
	 * A restitution coefficient set ot 0 implies that
	 * the exit velocity at a contact will be zero along the contact normal:
	 * it's as if you drop a ball but it doesn't bounce at all.
	 */

	restitution = 0.3;

	/**
	 * @type {RigidBody}
	 */
	character_rigid;

	/**
	 *
	 * @param {module} RAPIER
	 */
	constructor(RAPIER) {
		// make it a singleton, so we only have 1 threejs scene
		if (instance) {
			return instance;
		} else {
			instance = this;
		}

		const gravity = { x: 0.0, y: -9.81, z: 0.0 };
		/** @type {World} */
		this.world = new RAPIER.World(gravity);
		/** @type {ColliderDesc} */
		this.ColliderDesc = RAPIER.ColliderDesc;
		/** @type {RigidBodyDesc} */
		this.RigidBodyDesc = RAPIER.RigidBodyDesc;
		/** @type {CoefficientCombineRule} */
		this.CoefficientCombineRule = RAPIER.CoefficientCombineRule;
		/** @type {Ray} */
		this.Ray = new RAPIER.Ray({ x: 0, y: 0, z: 0 }, { x: 0, y: 1, z: 0 });
		/** @type {QueryFilterFlags} */
		this.QueryFilterFlags = RAPIER.QueryFilterFlags;
	}

	/**
	 * called in each `requestAnimationFrame`
	 */
	onFrameUpdate() {
		this.world.step();
	}

	destructor() {
		this.world.free();
	}

	/**
	 *
	 */
	createCharacter() {
		// rigidbody
		// @ts-ignore
		const rbDesc = this.RigidBodyDesc.kinematicPositionBased()
			.setTranslation(0, 0, 0)
			.enabledRotations(true, true, true)
			.setLinearDamping(0);

		this.character_rigid = this.world.createRigidBody(rbDesc);

		// collider, todo calculate this size by gltf model box
		// @ts-ignore
		const clDesc = this.ColliderDesc.cuboid(0.3, 0.9, 0.2)
			.setTranslation(0, 0.9, 0)
			.setFriction(this.friction)
			.setRestitution(this.restitution)
			.setMass(1);

		this.world.createCollider(clDesc, this.character_rigid);
	}

	removeCharacter() {
		// todo
		// this.world.removeRigidBody
	}
}
