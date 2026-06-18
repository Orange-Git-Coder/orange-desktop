/**
 * 全局类型定义
 *
 * 仅放跨模块共享的类型。
 * 模块内部类型放在对应模块的 types.ts 中。
 */

/** 分页请求参数 */
export interface PaginationParams {
    page: number;
    pageSize: number;
}

/** 分页响应 */
export interface PaginatedResponse<T> {
    items: T[];
    total: number;
    page: number;
    pageSize: number;
}

/** API 响应包裹 */
export interface ApiResponse<T = unknown> {
    code: number;
    message: string;
    data: T;
}