import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import { nodePolyfills } from 'vite-plugin-node-polyfills'
import { resolve } from 'path';

export default defineConfig({
	plugins: [sveltekit(), nodePolyfills({protocolImports: true}) ],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	},
	resolve: {
		alias: {
			'@components': resolve('./src/components'),
			'@lib': resolve('./src/lib'),
			'@stores': resolve('./src/stores'),
			'@routes': resolve('./src/routes'),
			'@interfaces': resolve('./src/interfaces'),
		}
	}
});
