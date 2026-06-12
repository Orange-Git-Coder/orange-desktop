/// <reference types="vite/client" />

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<object, object, unknown>;
  export default component;
}

/** 通用 API 响应类型 */
export interface ApiResponse<T = unknown> {
  code: number
  data: T
  message: string
}

/** 分页查询参数 */
export interface PaginationQuery {
  page: number
  pageSize: number
}

/** 分页响应数据 */
export interface PaginatedData<T> {
  list: T[]
  total: number
  page: number
  pageSize: number
}