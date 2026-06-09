<script setup lang="ts">
import { ref } from "vue"

const features = ref([
  {
    icon: "⚡",
    title: "Vue 3 + Vite",
    desc: "基于 Vue 3 Composition API 和 Vite 构建，极速热更新",
  },
  {
    icon: "🦀",
    title: "Tauri 2.x",
    desc: "轻量级桌面应用框架，Rust 驱动，安全高效",
  },
  {
    icon: "🎨",
    title: "SCSS 样式",
    desc: "SCSS 变量系统，支持暗黑模式自动切换",
  },
  {
    icon: "📡",
    title: "Axios 请求",
    desc: "请求拦截器 / 响应拦截器，统一错误处理",
  },
  {
    icon: "🧭",
    title: "Vue Router",
    desc: "页面路由管理，支持路由懒加载",
  },
  {
    icon: "🍍",
    title: "Pinia 状态",
    desc: "轻量状态管理，Setup Store 语法",
  },
])
</script>

<template>
  <div class="home">
    <section class="hero">
      <div class="hero-content">
        <h1 class="hero-title">
          <span class="gradient-text">🍊 Orange</span>
        </h1>
        <p class="hero-subtitle">Tauri + Vue 3 + TypeScript 桌面应用模板</p>
        <div class="hero-actions">
          <button class="btn btn-primary">开始使用</button>
          <button class="btn btn-outline">查看文档</button>
        </div>
      </div>
    </section>

    <section class="features">
      <h2 class="section-title">技术栈</h2>
      <div class="feature-grid">
        <div v-for="item in features" :key="item.title" class="feature-card">
          <span class="feature-icon">{{ item.icon }}</span>
          <h3 class="feature-title">{{ item.title }}</h3>
          <p class="feature-desc">{{ item.desc }}</p>
        </div>
      </div>
    </section>

    <footer class="footer">
      <p>Built with ❤️ using Tauri + Vue 3 + TypeScript</p>
    </footer>
  </div>
</template>

<style lang="scss" scoped>
// ==================== 变量 ====================
$gradient-start: #f093fb;
$gradient-end: #f5576c;
$gradient-orange: #fa8231;
$gradient-yellow: #fed330;

.home {
  min-height: 100vh;
  background: linear-gradient(135deg, #fafafa 0%, #fff5f5 50%, #fef3e2 100%);
}

// ==================== 暗黑模式 ====================
@media (prefers-color-scheme: dark) {
  .home {
    background: linear-gradient(135deg, #1a1a2e 0%, #16213e 50%, #0f3460 100%);
  }
}

// ==================== Hero ====================
.hero {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 100px 24px 60px;
  text-align: center;
}

.hero-content {
  max-width: 640px;
}

.hero-title {
  font-size: 56px;
  font-weight: 800;
  margin-bottom: 16px;
  letter-spacing: -1px;
}

.gradient-text {
  background: linear-gradient(135deg, $gradient-end, $gradient-orange);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.hero-subtitle {
  font-size: 20px;
  color: #666;
  margin-bottom: 36px;
  line-height: 1.6;

  @media (prefers-color-scheme: dark) {
    color: #aaa;
  }
}

.hero-actions {
  display: flex;
  gap: 12px;
  justify-content: center;
  flex-wrap: wrap;
}

// ==================== Buttons ====================
.btn {
  padding: 12px 28px;
  border-radius: 12px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  border: none;
  transition: all 0.3s ease;

  &-primary {
    background: linear-gradient(135deg, $gradient-end, $gradient-orange);
    color: #fff;
    box-shadow: 0 4px 15px rgba(245, 87, 108, 0.35);

    &:hover {
      transform: translateY(-2px);
      box-shadow: 0 6px 20px rgba(245, 87, 108, 0.45);
    }
  }

  &-outline {
    background: transparent;
    color: #555;
    border: 2px solid #ddd;
    @media (prefers-color-scheme: dark) {
      color: #ccc;
      border-color: #444;
    }

    &:hover {
      border-color: $gradient-orange;
      color: $gradient-orange;
    }
  }
}

// ==================== Features ====================
.features {
  padding: 60px 24px;
  max-width: 1100px;
  margin: 0 auto;
}

.section-title {
  text-align: center;
  font-size: 32px;
  font-weight: 700;
  margin-bottom: 40px;
  color: #333;
  @media (prefers-color-scheme: dark) {
    color: #eee;
  }
}

.feature-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 20px;
}

.feature-card {
  background: #fff;
  border-radius: 16px;
  padding: 28px 24px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
  transition: all 0.3s ease;
  border: 1px solid transparent;

  &:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 30px rgba(0, 0, 0, 0.1);
    border-color: $gradient-orange;
  }

  @media (prefers-color-scheme: dark) {
    background: #1e293b;
    border-color: #334155;
    &:hover {
      border-color: $gradient-orange;
    }
  }
}

.feature-icon {
  font-size: 36px;
  display: block;
  margin-bottom: 12px;
}

.feature-title {
  font-size: 18px;
  font-weight: 700;
  margin-bottom: 8px;
  color: #222;
  @media (prefers-color-scheme: dark) {
    color: #eee;
  }
}

.feature-desc {
  font-size: 14px;
  color: #777;
  line-height: 1.7;
  @media (prefers-color-scheme: dark) {
    color: #94a3b8;
  }
}

// ==================== Footer ====================
.footer {
  text-align: center;
  padding: 40px 24px;
  color: #999;
  font-size: 14px;
  @media (prefers-color-scheme: dark) {
    color: #666;
  }
}
</style>