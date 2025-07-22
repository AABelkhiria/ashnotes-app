import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig(({ mode }) => {
	const isProduction = mode === 'production' || process.env.TAURI_ENV_BUILD_CONTEXT === 'production';

	return {
		plugins: [sveltekit()],
		server: {
			proxy: {
				'/api': {
					target: 'http://localhost:3000',
					changeOrigin: true
				}
			}
		}
,
		envPrefix: ['VITE_', 'TAURI_'],
		base: isProduction ? '/' : './',
		build: {
			rollupOptions: {
				external: ['@tauri-apps/api/tauri']
			}
		}
	};
});
