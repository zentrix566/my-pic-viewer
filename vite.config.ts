import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// Tauri 环境下的 Vite 配置
// 参考: https://v2.tauri.app/start/frontend/vite/
export default defineConfig(async () => ({
  plugins: [vue()],

  // 使用相对路径，避免 Tauri 自定义协议下资源加载失败导致白屏
  base: './',

  // 阻止 vite 覆盖 Tauri 的错误处理并保持一致的开发端口
  clearScreen: false,

  server: {
    port: 1420,
    strictPort: true,
    host: false,
    hmr: {
      protocol: 'ws',
      host: 'localhost',
      port: 1421
    },
    watch: {
      // 忽略 Rust 相关文件，避免 Vite 无谓重载
      ignored: ['**/src-tauri/**']
    }
  },

  // 产物在 Tauri 打包时会被读取
  envPrefix: ['VITE_', 'TAURI_ENV_*'],

  build: {
    target: 'esnext',
    minify: 'esbuild',
    sourcemap: false
  }
}))
