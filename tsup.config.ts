import { defineConfig } from 'tsup';
import { wasmLoader } from 'esbuild-plugin-wasm';

export default defineConfig([
  {
    dts: true,
    entry: ['src/index.ts'],
    format: ['esm', 'cjs'],
    sourcemap: true,
    esbuildPlugins: [wasmLoader({ mode: 'embedded' })],
  },
]);
