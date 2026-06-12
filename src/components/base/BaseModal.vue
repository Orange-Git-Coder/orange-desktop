<script setup lang="ts">
defineProps<{
  visible: boolean
  title?: string
}>()

defineEmits<{
  (e: "close"): void
}>()
</script>

<template>
  <div v-if="visible" class="base-modal-overlay" @click.self="$emit('close')">
    <div class="base-modal">
      <div class="base-modal__header">
        <h3>{{ title }}</h3>
        <button class="base-modal__close" @click="$emit('close')">&times;</button>
      </div>
      <div class="base-modal__body">
        <slot />
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.base-modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.base-modal {
  background: #fff;
  border-radius: 12px;
  min-width: 400px;
  max-width: 90vw;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);

  &__header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 24px;
    border-bottom: 1px solid #eee;
  }

  &__close {
    background: none;
    border: none;
    font-size: 24px;
    cursor: pointer;
    color: #999;
  }

  &__body {
    padding: 24px;
  }
}
</style>