import { get, post } from "@/core/utils/request"

/** 读取目录内容 */
export function readDir(path: string) {
  return get("/api/fs/read-dir", { path })
}

/** 读取文件内容 */
export function readFile(path: string) {
  return get("/api/fs/read-file", { path })
}

/** 写入文件 */
export function writeFile(path: string, content: string) {
  return post("/api/fs/write-file", { path, content })
}