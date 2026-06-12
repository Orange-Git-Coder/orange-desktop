import { defineStore } from "pinia"
import { ref } from "vue"

/** 数据状态 Store */
export const useDataStore = defineStore("data", () => {
  const items = ref<unknown[]>([])
  const loading = ref(false)

  function setItems(list: unknown[]) {
    items.value = list
  }

  function setLoading(val: boolean) {
    loading.value = val
  }

  return {
    items,
    loading,
    setItems,
    setLoading,
  }
})