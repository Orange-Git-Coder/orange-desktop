/**
 * 全局常量
 *
 * 仅放跨模块使用的常量。
 * 模块内部常量放在对应模块的 constants.ts 中。
 */

/** 应用名称 */
export const APP_NAME = "Orange Desktop";

/** API 基础路径 */
export const API_BASE_URL = "/api";

/** 请求超时（毫秒） */
export const REQUEST_TIMEOUT = 10_000;

/** 默认主题 */
export const DEFAULT_THEME: "light" | "dark" = "light";

/** 侧边栏默认折叠 */
export const SIDEBAR_DEFAULT_COLLAPSED = false;