/**
 * https://kit.svelte.dev/docs/routing#layout
 * 
 *
    As well as `export function`, +page.js can export values that configure the page's behaviour:

    export const prerender = true or false or 'auto'
    export const ssr = true or false
    export const csr = true or false

    https://kit.svelte.dev/docs/page-options

    Data returned from a layout's load function is also available to all its child pages:

 * 
 */

/** @type {import('./$types').LayoutLoad} */

console.log("layout js loaded");
