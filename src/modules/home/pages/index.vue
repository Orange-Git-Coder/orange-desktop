<script setup lang="ts">
/**
 * 首页 —— Starter 展示页面
 */
import { ref, onMounted } from "vue";
import { useAppStore } from "@/stores/app";
import { APP_NAME } from "@/constants";
import request from "@/utils/request";
import { invoke } from "@tauri-apps/api/core";
import { Sunny, Setting, Connection, DataLine } from "@element-plus/icons-vue";

const appStore = useAppStore();

const greetMsg = ref("");
const systemInfo = ref("");
const httpResponse = ref("");

async function testGreet() {
    greetMsg.value = await invoke("greet", { name: "Orange" });
}

async function testSystemInfo() {
    systemInfo.value = JSON.stringify(
        await invoke("get_system_info"),
        null,
        2,
    );
}

async function testHttp() {
    try {
        const data = await request.get("/example");
        httpResponse.value = JSON.stringify(data, null, 2);
    } catch {
        httpResponse.value = "请求失败（预期行为，无后端服务）";
    }
}

onMounted(() => {
    testGreet();
    testSystemInfo();
    testHttp();
});
</script>

<template>
    <div class="home-page">
        <el-card class="header-card">
            <h1>🍊 {{ APP_NAME }}</h1>
            <p class="subtitle">Tauri v2 + Vue 3 + TypeScript Starter Template</p>
        </el-card>

        <el-row :gutter="20">
            <el-col :span="12">
                <el-card>
                    <template #header>
                        <div class="card-title">
                            <el-icon><Connection /></el-icon>
                            <span>Tauri IPC 调用</span>
                        </div>
                    </template>
                    <el-tag type="success" size="large">{{ greetMsg || "加载中..." }}</el-tag>
                </el-card>
            </el-col>

            <el-col :span="12">
                <el-card>
                    <template #header>
                        <div class="card-title">
                            <el-icon><Sunny /></el-icon>
                            <span>主题切换</span>
                        </div>
                    </template>
                    <el-switch
                        :model-value="appStore.theme === 'dark'"
                        active-text="暗色"
                        inactive-text="亮色"
                        @change="appStore.toggleTheme()"
                    />
                </el-card>
            </el-col>
        </el-row>

        <el-row :gutter="20" style="margin-top: 20px">
            <el-col :span="12">
                <el-card>
                    <template #header>
                        <div class="card-title">
                            <el-icon><Setting /></el-icon>
                            <span>系统信息（Rust）</span>
                        </div>
                    </template>
                    <el-input
                        :model-value="systemInfo"
                        type="textarea"
                        :rows="3"
                        readonly
                    />
                </el-card>
            </el-col>

            <el-col :span="12">
                <el-card>
                    <template #header>
                        <div class="card-title">
                            <el-icon><DataLine /></el-icon>
                            <span>axios HTTP 请求</span>
                        </div>
                    </template>
                    <el-input
                        :model-value="httpResponse"
                        type="textarea"
                        :rows="3"
                        readonly
                    />
                </el-card>
            </el-col>
        </el-row>
    </div>
</template>

<style lang="scss" scoped>
.home-page {
    max-width: 800px;
    margin: 0 auto;
}

.header-card {
    margin-bottom: 20px;
    text-align: center;
}

.header-card :deep(h1) {
    margin: 0 0 8px;
}

.subtitle {
    margin: 0;
    color: #909399;
    font-size: 14px;
}

.card-title {
    display: flex;
    align-items: center;
    gap: 8px;
    font-weight: 600;
}
</style>