import { get, post, put, del } from "@/core/utils/request"

/** 获取数据列表 */
export function fetchDataList(params?: Record<string, unknown>) {
  return get("/api/data", params)
}

/** 创建数据 */
export function createData(data: unknown) {
  return post("/api/data", data)
}

/** 更新数据 */
export function updateData(id: string, data: unknown) {
  return put(`/api/data/${id}`, data)
}

/** 删除数据 */
export function deleteData(id: string) {
  return del(`/api/data/${id}`)
}