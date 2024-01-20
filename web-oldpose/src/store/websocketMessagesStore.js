import { writable } from "svelte/store";

// Initialize with an empty array
const websocket_messages = writable([]);

export default websocket_messages;
