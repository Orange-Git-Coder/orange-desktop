<script setup lang="ts">
defineProps<{
  username: string
  size?: number
}>()
</script>

<template>
  <div class="user-avatar" :style="{ width: size + 'px', height: size + 'px' }">
    {{ username.charAt(0).toUpperCase() }}
  </div>
</template>

<style lang="scss" scoped>
.user-avatar {
  display: flex;
  justify-content: center;
  align-items: center;
  border-radius: 50%;
  background: linear-gradient(135deg, #667eea, #764ba2);
  color: #fff;
  font-weight: 700;
  user-select: none;
}
</style>