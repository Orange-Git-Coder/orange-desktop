import { get } from "@/core/utils/request"

/** 获取系统信息 */
export function getSystemInfo() {
  return get("/api/system/info")
}