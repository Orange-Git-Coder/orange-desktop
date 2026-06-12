<script setup lang="ts">
// 空白布局 —— 用于登录、注册等独立页面
</script>

<template>
  <div class="blank-layout">
    <slot />
  </div>
</template>

<style lang="scss" scoped>
.blank-layout {
  min-height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
  background: #f8fafc;
}
</style>