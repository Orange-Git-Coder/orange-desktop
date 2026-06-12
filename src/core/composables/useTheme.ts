import { ref } from "vue"

const theme = ref<"light" | "dark">("light")

export function useTheme() {
  const prefersDark = window.matchMedia("(prefers-color-scheme: dark)")

  const applyTheme = () => {
    theme.value = prefersDark.matches ? "dark" : "light"
    document.documentElement.setAttribute("data-theme", theme.value)
  }

  applyTheme()
  prefersDark.addEventListener("change", applyTheme)

  return { theme }
}