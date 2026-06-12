<script setup lang="ts">
// 默认布局 —— 包含侧边栏 + 顶部导航
</script>

<template>
  <div class="default-layout">
    <aside class="default-layout__sidebar">
      <div class="sidebar-brand">🍊 Orange</div>
      <nav class="sidebar-nav">
        <router-link to="/">首页</router-link>
        <router-link to="/about">关于</router-link>
      </nav>
    </aside>
    <main class="default-layout__main">
      <slot />
    </main>
  </div>
</template>

<style lang="scss" scoped>
.default-layout {
  display: flex;
  min-height: 100vh;

  &__sidebar {
    width: 240px;
    background: #1e293b;
    color: #fff;
    padding: 24px 0;
  }

  &__main {
    flex: 1;
    background: #f8fafc;
  }
}

.sidebar-brand {
  padding: 0 24px 24px;
  font-size: 20px;
  font-weight: 700;
}

.sidebar-nav {
  display: flex;
  flex-direction: column;

  a {
    padding: 12px 24px;
    color: #94a3b8;
    text-decoration: none;
    transition: all 0.2s;

    &:hover {
      color: #fff;
      background: rgba(255, 255, 255, 0.05);
    }

    &.router-link-active {
      color: #fff;
      background: rgba(255, 255, 255, 0.1);
    }
  }
}
</style>