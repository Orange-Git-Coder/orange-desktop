/**
 * 路由配置
 *
 * 规范：
 * 1. 路由仅做页面映射，不包含业务逻辑
 * 2. 业务页面统一通过 modules 懒加载
 * 3. 路由路径使用 kebab-case
 */
import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: "/",
            component: () => import("@/layouts/default.vue"),
            children: [
                {
                    path: "",
                    name: "home",
                    component: () => import("@/modules/home/pages/index.vue"),
                    meta: { title: "首页" },
                },
            ],
        },
    ],
});

export default router;