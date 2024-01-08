import { writable } from 'svelte/store';

// Initialize with an empty array
const timeline = writable([]);

export default timeline;