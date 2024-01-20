import { writable } from "svelte/store";

const diva = writable(null);
const shadow = writable(null);
const scenery = writable(null);

export { diva, shadow, scenery };