<script setup lang="ts">
defineProps<{
  variant?: "primary" | "outline" | "ghost"
  size?: "sm" | "md" | "lg"
  disabled?: boolean
}>()
</script>

<template>
  <button
    class="base-button"
    :class="[`base-button--${variant || 'primary'}`, `base-button--${size || 'md'}`]"
    :disabled="disabled"
  >
    <slot />
  </button>
</template>

<style lang="scss" scoped>
.base-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-weight: 600;
  transition: all 0.2s ease;

  &--sm { padding: 6px 12px; font-size: 12px; }
  &--md { padding: 10px 20px; font-size: 14px; }
  &--lg { padding: 14px 28px; font-size: 16px; }

  &--primary {
    background: #396cd8;
    color: #fff;
    &:hover { background: #4a7fe0; }
  }

  &--outline {
    background: transparent;
    border: 2px solid #396cd8;
    color: #396cd8;
    &:hover { background: #396cd8; color: #fff; }
  }

  &--ghost {
    background: transparent;
    color: #396cd8;
    &:hover { background: rgba(57, 108, 216, 0.1); }
  }

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
}
</style>