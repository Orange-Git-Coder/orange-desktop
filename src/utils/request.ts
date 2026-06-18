/**
 * Axios 请求封装
 *
 * 职责：统一拦截器、错误处理、请求/响应转换。
 * 所有模块的 api.ts 通过此文件发起 HTTP 请求。
 */
import axios from "axios";
import type { AxiosInstance, AxiosResponse } from "axios";

// ==================== 创建实例 ====================

const instance: AxiosInstance = axios.create({
    baseURL: "/api",
    timeout: 10_000,
    headers: { "Content-Type": "application/json" },
});

// ==================== 请求拦截器 ====================

instance.interceptors.request.use(
    (config) => {
        // 可在此处统一注入 token
        // const token = localStorage.getItem("token");
        // if (token) config.headers.Authorization = `Bearer ${token}`;
        return config;
    },
    (error) => Promise.reject(error),
);

// ==================== 响应拦截器 ====================

instance.interceptors.response.use(
    (response: AxiosResponse) => {
        // 直接返回 data，调用方无需 .data
        return response.data;
    },
    (error) => {
        // 统一错误提示
        const message = error.response?.data?.message
            || error.message
            || "网络请求失败";
        console.error(`[Request Error] ${message}`);
        return Promise.reject(error);
    },
);

// ==================== 对外暴露 ====================

export default instance;