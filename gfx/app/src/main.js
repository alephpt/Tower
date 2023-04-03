import App from './App.svelte';
import init from '../../pkg/index.js';

init().then(() => {
	console.log('init done');
});

const app = new App({
	target: document.body,
	props: {
		words: 'PROPS!',
	},

});

export default app;