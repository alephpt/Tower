import App from './App.svelte';

const app = new App({
	target: document.body,
	props: {
		words: 'Svelte is running!',
	},

});

export default app;