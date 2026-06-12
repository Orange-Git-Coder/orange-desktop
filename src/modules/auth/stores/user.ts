import { defineStore } from "pinia"
import { ref } from "vue"

/** 用户状态 Store */
export const useUserStore = defineStore("user", () => {
  const token = ref<string | null>(null)
  const username = ref<string>("")

  function setToken(t: string) {
    token.value = t
  }

  function setUsername(name: string) {
    username.value = name
  }

  function logout() {
    token.value = null
    username.value = ""
  }

  const isLoggedIn = () => !!token.value

  return {
    token,
    username,
    setToken,
    setUsername,
    logout,
    isLoggedIn,
  }
})