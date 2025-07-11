import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		proxy: {
			'/api': {
				target: 'http://localhost:3000',
				changeOrigin: true
			}
		}
	},
	envPrefix: ['VITE_', 'TAURI_'],
	build: {
		rollupOptions: {
			external: ['@tauri-apps/api/tauri']
		}
	}
});
