import { writable } from "svelte/store";

const diva = writable(null);
const shadow = writable(null);

export { diva, shadow };