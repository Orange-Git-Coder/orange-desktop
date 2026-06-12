import axios from "axios"
import type { AxiosInstance, AxiosRequestConfig, InternalAxiosRequestConfig, AxiosResponse } from "axios"
import type { ApiResponse } from "@/core/types/global"

const instance: AxiosInstance = axios.create({
  baseURL: import.meta.env.VITE_API_BASE_URL || "",
  timeout: 10000,
  headers: {
    "Content-Type": "application/json",
  },
})

// 请求拦截器
instance.interceptors.request.use(
  (config: InternalAxiosRequestConfig) => {
    // 可在此添加 token
    // const token = localStorage.getItem('token')
    // if (token) config.headers.Authorization = `Bearer ${token}`
    return config
  },
  (error) => Promise.reject(error)
)

// 响应拦截器
instance.interceptors.response.use(
  (response: AxiosResponse<ApiResponse>) => {
    const { data } = response
    if (data.code !== 0) {
      console.warn(`[request] ${data.message}`)
      return Promise.reject(new Error(data.message || "请求失败"))
    }
    return response
  },
  (error) => {
    console.error(`[request] ${error.message}`)
    return Promise.reject(error)
  }
)

/** GET 请求 */
export function get<T = unknown>(
  url: string,
  params?: Record<string, unknown>,
  config?: AxiosRequestConfig
): Promise<AxiosResponse<ApiResponse<T>>> {
  return instance.get(url, { params, ...config })
}

/** POST 请求 */
export function post<T = unknown>(
  url: string,
  data?: unknown,
  config?: AxiosRequestConfig
): Promise<AxiosResponse<ApiResponse<T>>> {
  return instance.post(url, data, config)
}

/** PUT 请求 */
export function put<T = unknown>(
  url: string,
  data?: unknown,
  config?: AxiosRequestConfig
): Promise<AxiosResponse<ApiResponse<T>>> {
  return instance.put(url, data, config)
}

/** DELETE 请求 */
export function del<T = unknown>(
  url: string,
  config?: AxiosRequestConfig
): Promise<AxiosResponse<ApiResponse<T>>> {
  return instance.delete(url, config)
}

export default instance