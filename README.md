# orange-desktop

基于 Tauri v2 + Vue 3 + TypeScript 的桌面应用。

## 环境要求

- [Node.js](https://nodejs.org/) >= 18
- [pnpm](https://pnpm.io/installation) 包管理器
- [Rust](https://www.rust-lang.org/tools/install) 工具链

## 快速开始

### 1. 安装前端依赖

```bash
pnpm install
```

### 2. 开发模式运行

```bash
pnpm tauri dev
```

这将同时启动 Vite 开发服务器（前端热更新）和 Tauri 桌面窗口（Rust 后端）。

### 3. 生产构建

```bash
pnpm tauri build
```

构建产物位于 `src-tauri/target/release/bundle/` 目录下。

## 推荐 IDE 配置

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
