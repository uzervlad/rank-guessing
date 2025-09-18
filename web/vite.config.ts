import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		proxy: {
			'/api': {
				target: 'http://localhost:3999',
				changeOrigin: true,
				rewrite: path => path.replace(/^\/api/, ''),
				ws: true,
			},
		},
	},
});
