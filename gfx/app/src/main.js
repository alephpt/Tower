import App from './App.svelte';
import init from '../../pkg/index.js';

init().then(() => {
	console.log('WGPU Loaded!');
});

const app = new App({
	target: document.body,
	props: {
		words: 'Svelte is running!',
	},

});

export default app;