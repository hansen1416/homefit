import { writable } from "svelte/store";

// Initialize with an empty array
const animation_queue = writable([]);

export default animation_queue;
