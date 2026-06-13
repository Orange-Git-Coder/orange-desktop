import { createRouter, createWebHistory } from "vue-router"
import type { RouteRecordRaw } from "vue-router"
// vite-plugin-pages 自动根据 src/pages 目录结构生成路由表
import generatedRoutes from "~pages"

/**
 * 文件系统路由：
 *   src/pages/auth/login.vue      → /auth/login
 *   src/pages/auth/register.vue   → /auth/register
 *   src/pages/database/data-manager.vue → /database/data-manager
 *   src/pages/file-system/file-explorer.vue → /file-system/file-explorer
 *   src/pages/system/system-info.vue → /system/system-info
 *   src/pages/index.vue           → /
 *
 * 命名规则遵循 Nuxt 约定：
 *   - kebab-case 文件名 → kebab-case URL 路径
 *   - 目录层级 → 路径前缀
 *   - index.vue → /
 */
const routes: RouteRecordRaw[] = generatedRoutes

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router