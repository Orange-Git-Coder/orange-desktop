/**
 * 应用全局 Store
 *
 * 职责：
 * 1. 主题管理（亮色/暗色）
 * 2. 侧边栏折叠状态
 * 3. 不包含任何业务模块的状态
 */
import { defineStore } from "pinia";
import { ref } from "vue";

export type Theme = "light" | "dark";

export const useAppStore = defineStore("app", () => {
    // ==================== 状态 ====================
    const sidebarCollapsed = ref(false);
    const theme = ref<Theme>(
        (localStorage.getItem("theme") as Theme | null) ?? "light",
    );

    const prefersDark = window.matchMedia("(prefers-color-scheme: dark)");

    // ==================== 私有方法 ====================
    function applyTheme() {
        document.documentElement.setAttribute("data-theme", theme.value);
        localStorage.setItem("theme", theme.value);
    }

    // ==================== 公开方法 ====================
    function toggleSidebar() {
        sidebarCollapsed.value = !sidebarCollapsed.value;
    }

    function setTheme(t: Theme) {
        theme.value = t;
        applyTheme();
    }

    function toggleTheme() {
        theme.value = theme.value === "light" ? "dark" : "light";
        applyTheme();
    }

    // ==================== 初始化 ====================
    applyTheme();
    prefersDark.addEventListener("change", () => {
        theme.value = prefersDark.matches ? "dark" : "light";
        applyTheme();
    });

    return {
        sidebarCollapsed,
        theme,
        toggleSidebar,
        setTheme,
        toggleTheme,
    };
});