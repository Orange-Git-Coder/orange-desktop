# 🍊 Orange Desktop — Tauri Desktop Starter Template

基于 **Tauri v2** + **Vue 3** + **TypeScript** 的桌面应用 Starter Template。

## 技术栈

| 层       | 技术               | 用途                     |
| -------- | ------------------ | ------------------------ |
| 桌面框架 | Tauri 2.x          | Rust 驱动，轻量高效      |
| 前端     | Vue 3 + TypeScript | Composition API           |
| 构建     | Vite 7             | 极速 HMR                 |
| 样式     | SCSS               | 模块化样式               |
| 状态管理 | Pinia 3            | Setup Store 语法         |
| 路由     | Vue Router 5       | 懒加载路由               |
| HTTP     | axios              | 拦截器 + 请求封装        |
| 运行时   | Deno 2             | 零配置 TypeScript        |
| 后端     | Rust               | Tauri Command + Service  |

## 目录结构

```
orange-desktop/
│
├── src/                              # Vue 3 前端
│   ├── router/index.ts               # 路由配置（懒加载 modules）
│   ├── stores/app.ts                 # 全局状态（主题 / 侧边栏）
│   ├── utils/
│   │   ├── format.ts                 # 格式化工具
│   │   └── request.ts                # axios 请求封装（拦截器）
│   ├── types/global.d.ts            # 全局类型
│   ├── constants/index.ts            # 全局常量
│   ├── layouts/
│   │   ├── default.vue               # 默认布局（侧边栏 + 主内容）
│   │   └── blank.vue                 # 空白布局
│   ├── components/base/              # 全局 UI 组件
│   ├── modules/                      # 业务模块
│   │   └── home/pages/index.vue      # 首页（展示页）
│   ├── app.vue
│   └── main.ts
│
├── src-tauri/src/                    # Rust 后端
│   ├── main.rs                       # 启动入口
│   ├── lib.rs                        # 模块注册
│   ├── error.rs                      # 统一错误处理
│   ├── state.rs                      # 全局状态
│   ├── commands/                     # Tauri Command 层
│   │   ├── auth.rs                   # 鉴权命令（greet）
│   │   ├── file.rs                   # 文件命令
│   │   └── system.rs                 # 系统命令
│   ├── services/                     # 业务逻辑层
│   │   ├── file_service.rs
│   │   └── system_service.rs
│   ├── models/                       # 数据模型
│   │   └── file.rs
│   └── tray/                         # 系统托盘
│
├── public/                           # 静态资源
├── deno.json                         # Deno 配置
├── tsconfig.json                     # TypeScript 配置
├── vite.config.ts                    # Vite 配置
└── README.md
```

## 目录职责

| 目录          | 职责                                     |
| ------------- | ---------------------------------------- |
| `router/`     | 仅放路由配置                             |
| `stores/`     | 仅放全局状态（主题、侧边栏）             |
| `utils/`      | 仅放通用工具函数                         |
| `types/`      | 仅放跨模块共享类型                       |
| `constants/`  | 仅放跨模块共享常量                       |
| `layouts/`    | 仅放布局组件                             |
| `components/` | 仅放全局 UI 组件                         |
| `modules/`    | 所有业务功能，每个模块一个目录，零耦合   |
| `commands/`   | Tauri Command —— 接收前端调用 → 转发 Service |
| `services/`   | 业务逻辑 —— 可被多个 Command 复用        |
| `models/`     | 纯数据结构，不含业务逻辑                 |
| `tray/`       | 系统托盘                                 |
| `error.rs`    | 统一错误类型                             |
| `state.rs`    | 全局共享状态                             |

## 快速开始

```bash
# 1. 安装依赖
deno install

# 2. 开发模式
deno task tauri dev

# 3. 生产构建
deno task tauri build
```

## 模块开发规范

### 模块结构

```
modules/<module_name>/
├── pages/          # 页面组件（必选）
├── components/     # 模块内部组件
├── api.ts          # 网络请求
├── store.ts        # 模块状态
├── types.ts        # 模块类型
├── constants.ts    # 模块常量
├── utils.ts        # 模块工具
└── index.ts        # 入口，对外暴露公共 API
```

### 模块引用规则

```
✅ 允许：modules/order → @/utils/format   （引用全局工具）
❌ 禁止：modules/order → ../user/store   （跨模块引用）
```

### 新增模块流程（前端）

```
1. mkdir src/modules/download/pages
2. 创建页面 src/modules/download/pages/index.vue
3. 在 router/index.ts 添加路由
4. 在 layouts/default.vue 的 navItems 添加导航项
```

### 新增模块流程（后端）

```
1. 创建 Service：src-tauri/src/services/download_service.rs
2. 注册 Service：编辑 services/mod.rs，添加 pub mod download_service;
3. 创建 Command：src-tauri/src/commands/download.rs
4. 注册 Command：编辑 commands/mod.rs，添加 pub mod download;
5. 注册 handler：编辑 lib.rs，在 generate_handler![] 中添加
```

### 删除模块

```
1. 删除前端目录 rm -rf src/modules/download/
2. 移除路由和导航
3. 删除后端 Command + Service
4. 编辑 commands/mod.rs、services/mod.rs、lib.rs 移除对应行
```

## 命名规范

| 类型        | 规范       | 示例              |
| ----------- | ---------- | ----------------- |
| 目录        | snake_case | `file/`           |
| Vue 组件    | PascalCase | `FileList.vue`    |
| TS 文件     | 简短名     | `api.ts` `store.ts` |
| Rust 文件   | snake_case | `file_service.rs` |
| 路由 path   | kebab-case | `/user-profile`   |
| 路由 name   | camelCase  | `userProfile`     |
| TS 变量     | camelCase  | `userName`        |
| Rust 函数   | snake_case | `file_list`       |
| Pinia Store | `use` + PascalCase + `Store` | `useFileStore` |

## 架构规范

### Rust 分层

```
前端 invoke("command_name")  →  commands/*.rs  （接收 + 转发）
                                     ↓
                               services/*.rs  （业务逻辑）
                                     ↓
                               System API / 文件系统
```

- **Command** 严禁写业务逻辑，只做参数接收和转发
- **Service** 不依赖 Tauri 类型，纯 Rust，方便测试
- **错误** 统一用 `AppError`，Command 层转为 `String` 返回

### 状态管理

| 层级   | 位置               | 示例          | 范围       |
| ------ | ------------------ | ------------- | ---------- |
| 全局   | `stores/app.ts`    | `useAppStore` | 所有模块   |
| 模块级 | `modules/*/store.ts` | `useFileStore` | 仅本模块 |

### axios 请求

```
页面 → store.ts → api.ts → @/utils/request.ts → axios 实例
```

- **api.ts**：仅定义请求函数
- **store.ts**：调用 api.ts，管理状态和缓存
- **request.ts**：axios 实例 + 拦截器 + 错误处理

## 依赖说明

### deno.json

| 依赖                     | 用途                    |
| ------------------------ | ----------------------- |
| `@tauri-apps/api`        | Tauri IPC               |
| `@tauri-apps/plugin-opener` | 系统默认应用打开     |
| `pinia`                  | 状态管理                |
| `vue`                    | 前端框架                |
| `vue-router`             | 路由                    |
| `sass`                   | SCSS 编译               |
| `axios`                  | HTTP 请求（拦截器）     |
| `vite` / `@vitejs/plugin-vue` | 构建工具         |
| `@tauri-apps/cli`        | Tauri CLI              |
| `typescript`             | 类型检查                |

### Cargo.toml

| Crate                | 用途                     |
| -------------------- | ------------------------ |
| `tauri`              | Tauri 核心               |
| `tauri-plugin-opener` | 系统默认应用打开         |
| `serde` / `serde_json` | 序列化                 |
| `tauri-build`        | 构建工具                 |

## License

MIT