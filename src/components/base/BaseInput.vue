<script setup lang="ts">
defineProps<{
  modelValue: string
  placeholder?: string
  type?: string
  disabled?: boolean
}>()

defineEmits<{
  (e: "update:modelValue", value: string): void
}>()
</script>

<template>
  <input
    class="base-input"
    :type="type || 'text'"
    :value="modelValue"
    :placeholder="placeholder"
    :disabled="disabled"
    @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
  />
</template>

<style lang="scss" scoped>
.base-input {
  padding: 10px 14px;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 14px;
  outline: none;
  transition: border-color 0.2s ease;
  width: 100%;

  &:focus {
    border-color: #396cd8;
  }

  &:disabled {
    background: #f5f5f5;
    cursor: not-allowed;
  }
}
</style>