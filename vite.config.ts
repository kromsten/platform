import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import { nodePolyfills } from 'vite-plugin-node-polyfills'
import { resolve } from 'path';

export default defineConfig({
	plugins: [sveltekit(), nodePolyfills({protocolImports: true}) ],
	test: {
		include: ['*tests*/**/*.{test,spec}.{js,ts}'],
		testTimeout: 20000
	},
	resolve: {
		alias: {
			'@components': resolve('./src/components'),
			'@lib': resolve('./src/lib'),
			'@routes': resolve('./src/routes'),
			'@interfaces': resolve('./src/interfaces'),
		}
	}
});
