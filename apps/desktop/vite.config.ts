import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  root: path.resolve(__dirname, 'frontend'),
  publicDir: path.resolve(__dirname, 'frontend/public'),
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './frontend'),
      '@lib': path.resolve(__dirname, './frontend/lib'),
      '@features': path.resolve(__dirname, './frontend/features'),
      '@components': path.resolve(__dirname, './frontend/components'),
      '@styles': path.resolve(__dirname, './frontend/styles'),
    },
  },
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: true,
    open: true, // Automatically open the browser
  },
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    target: ['es2021', 'chrome100', 'safari13'],
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    sourcemap: !!process.env.TAURI_DEBUG,
    outDir: path.resolve(__dirname, 'dist'),
  },
});
