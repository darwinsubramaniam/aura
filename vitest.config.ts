import { defineConfig, mergeConfig } from 'vitest/config';
import viteConfig from './vite.config';

export default defineConfig(async (env) => {
    const defaultConfig = typeof viteConfig === 'function' ? await viteConfig(env) : viteConfig;
    return mergeConfig(defaultConfig, {
        test: {
            globals: true,
            environment: 'jsdom',
            include: ['**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}'],
        },
    });
});
