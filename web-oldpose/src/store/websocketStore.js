import { writable } from "svelte/store";

const websocket_state = writable(0);

export default websocket_state;