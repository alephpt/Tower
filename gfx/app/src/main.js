import App from './App.svelte';
import init from '../../pkg/gfx.js';

const app = new App({
	target: document.body,
	props: {
		words: 'Svelte is running!',
		// pass in the wasm module
		wasm: init,
	},

});

export default app;