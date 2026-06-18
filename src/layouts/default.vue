<script setup lang="ts">
import { useAppStore } from "@/stores/app";

const app = useAppStore();
const navItems = [{ path: "/", title: "首页", icon: "🏠" }];
</script>

<template>
    <el-container style="min-height: 100vh">
        <el-aside :width="app.sidebarCollapsed ? '64px' : '240px'">
            <el-menu
                :collapse="app.sidebarCollapsed"
                router
                :default-active="$route.path"
                background-color="#1e293b"
                text-color="#94a3b8"
                active-text-color="#fff"
                style="height: 100%"
            >
                <div style="padding: 20px 16px; color: #fff; font-weight: bold; font-size: 18px; white-space: nowrap; overflow: hidden">
                    <span v-if="!app.sidebarCollapsed">🍊 Orange Desktop</span>
                    <span v-else>🍊</span>
                </div>

                <el-menu-item v-for="item in navItems" :key="item.path" :index="item.path">
                    <span>{{ item.icon }} {{ item.title }}</span>
                </el-menu-item>
            </el-menu>
        </el-aside>

        <el-container>
            <el-header style="display: flex; align-items: center; background: #fff; border-bottom: 1px solid #e4e7ed; padding: 0 20px">
                <el-button text @click="app.toggleSidebar()">
                    {{ app.sidebarCollapsed ? '展开' : '折叠' }}
                </el-button>
            </el-header>
            <el-main style="background: #f5f7fa">
                <router-view />
            </el-main>
        </el-container>
    </el-container>
</template>