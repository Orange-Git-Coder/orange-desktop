import { get, post } from "@/core/utils/request"

/** 登录 */
export function login(username: string, password: string) {
  return post("/api/auth/login", { username, password })
}

/** 注册 */
export function register(username: string, password: string, email?: string) {
  return post("/api/auth/register", { username, password, email })
}

/** 获取当前用户信息 */
export function getUserInfo() {
  return get("/api/auth/me")
}