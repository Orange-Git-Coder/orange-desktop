<script setup lang="ts">
defineProps<{
  title: string
  description?: string
}>()
</script>

<template>
  <div class="page-header">
    <h1 class="page-header__title">{{ title }}</h1>
    <p v-if="description" class="page-header__desc">{{ description }}</p>
  </div>
</template>

<style lang="scss" scoped>
.page-header {
  padding: 32px 24px 24px;

  &__title {
    font-size: 24px;
    font-weight: 700;
    color: #222;
  }

  &__desc {
    margin-top: 8px;
    font-size: 14px;
    color: #888;
  }
}
</style>