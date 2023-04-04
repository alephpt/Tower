import App from './App.svelte';
import init from '../../pkg/gfx.js';

init().then(() => console.log("WASM Loaded"));

const app = new App({
	target: document.body,
	props: {
		words: 'Svelte is running!',
		// pass in the wasm module
		wasm: init,
	},

});

export default app;