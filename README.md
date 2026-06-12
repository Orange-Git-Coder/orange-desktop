# 🍊 Orange Desktop

基于 **Tauri v2** + **Vue 3** + **TypeScript** 的桌面应用框架，采用模块化架构设计，前后端模块一一对应，核心骨架与可选模块分离，可按需裁剪。

## 技术栈

| 层 | 技术 | 说明 |
|---|------|------|
| 桌面框架 | Tauri 2.x | Rust 驱动，轻量高效 |
| 前端 | Vue 3 + TypeScript | Composition API + `<script setup>` |
| 构建 | Vite 6 | 极速 HMR |
| 状态管理 | Pinia 3 | Setup Store 语法 |
| 路由 | Vue Router 5 | 懒加载路由 |
| 样式 | SCSS | 变量 + 暗黑模式 |
| HTTP | Axios | 请求/响应拦截器 |
| 运行时 | Deno 2 | 零配置 TypeScript，原生 npm 兼容 |

## 项目结构

```
orange-desktop/
├── src-tauri/                     # 🦀 Rust 后端
│   └── src/
│       ├── core/                  # 🔥 核心骨架（永不删除）
│       │   ├── app_state.rs       #   全局状态
│       │   ├── error.rs           #   错误类型
│       │   └── logger.rs          #   日志器
│       ├── modules/               # 🧩 可选模块（按需保留/删除）
│       │   ├── auth/              #   认证模块
│       │   ├── database/          #   数据库模块
│       │   ├── file_system/       #   文件系统模块
│       │   └── system/            #   系统信息模块
│       ├── utils/                 # 🛠️ 工具函数
│       │   ├── crypto.rs          #   加密工具（可选）
│       │   ├── fs.rs              #   文件工具（可选）
│       │   └── string.rs          #   字符串工具（核心）
│       ├── tray/                  #   托盘图标
│       │   ├── mod.rs             #     主入口
│       │   ├── menu.rs            #     菜单构建
│       │   └── events.rs          #     事件处理
│       └── lib.rs                 #   入口点
│
├── src/                           # 💚 Vue 3 前端
│   ├── core/                      # 🔥 核心骨架（永不删除）
│   │   ├── router/                #   路由配置
│   │   ├── stores/                #   全局 Store
│   │   ├── utils/                 #   工具函数 (request + format)
│   │   ├── composables/           #   组合式函数 (useTheme)
│   │   ├── styles/                #   全局样式 + 变量
│   │   └── types/                 #   全局类型定义
│   ├── modules/                   # 🧩 可选模块（与后端一一对应）
│   │   ├── auth/                  #   views/ stores/ api/ components/
│   │   ├── database/              #   views/ stores/ api/
│   │   ├── file_system/           #   views/ api/
│   │   └── system/                #   views/ api/
│   ├── components/                # 公共组件
│   │   ├── base/                  #   基础组件 (Button, Input, Modal, Table)
│   │   └── business/              #   业务组件 (PageHeader)
│   ├── layouts/                   # 布局组件
│   │   ├── DefaultLayout.vue      #   默认布局（侧边栏 + 内容）
│   │   └── BlankLayout.vue        #   空白布局（登录页等）
│   └── views/                     # 核心页面
│       ├── home/                  #   首页
│       └── about/                 #   关于页
│
├── deno.json                      # Deno 配置（依赖 + 任务）
├── public/                        # 静态资源
├── .env.example                   # 环境变量示例
└── README.md
```

## 环境要求

- [Deno](https://deno.com/) >= 2.0
- [Rust](https://www.rust-lang.org/tools/install) 工具链

## 快速开始

### 1. 安装依赖

```bash
deno install
```

Deno 会根据 `deno.json` 中的 `imports` 字段自动解析并安装所有 npm 依赖。

### 2. 开发模式

```bash
deno task tauri dev
```

同时启动 Vite 开发服务器（前端热更新）和 Tauri 桌面窗口。

### 3. 生产构建

```bash
deno task tauri build
```

构建产物位于 `src-tauri/target/release/bundle/`。

## 可用任务

| 命令 | 说明 |
|------|------|
| `deno task dev` | 启动 Vite 开发服务器 |
| `deno task build` | 类型检查 + 生产构建 |
| `deno task preview` | 预览生产构建 |
| `deno task tauri dev` | Tauri 开发模式 |
| `deno task tauri build` | Tauri 生产构建 |

## 模块管理

项目采用 **核心 + 可选模块** 架构：

- `core/` —— 核心骨架，**永不删除**，包含路由、Store、HTTP 请求、全局样式等基础设施
- `modules/` —— 可选业务模块，**按需保留或删除**

### 删除模块

每个模块有前端 + 后端两组文件，并可能散布在多处引用。按以下步骤删除任意模块：

**通用步骤：**

1. 删除前端模块目录
2. 删除后端模块目录  
3. 编辑 `src-tauri/src/modules/mod.rs`，移除对应 `pub mod xxx;`
4. 编辑 `src-tauri/src/lib.rs`，移除对应命令注册
5. 编辑 `src/core/router/index.ts`，移除对应路由
6. 编辑 `src/layouts/DefaultLayout.vue`，移除对应侧边栏菜单项
7. 编辑 `deno.json`，移除该模块依赖的 `imports` 条目

#### 模块：auth（认证）

| 删除目录/文件 | 说明 |
|---|---|
| `src/modules/auth/` | 前端：views/stores/api/components |
| `src-tauri/src/modules/auth/` | 后端：commands/handlers/models/error |

| 编辑文件 | 操作 |
|---|---|
| `src-tauri/src/modules/mod.rs` | 删除 `pub mod auth;` |
| `src-tauri/src/lib.rs` | 删除 `auth::commands::*` 对应的命令注册 |
| `src/core/router/index.ts` | 删除 `/login`、`/register` 路由 |
| `src/layouts/DefaultLayout.vue` | 删除侧边栏中 auth 相关菜单项 |

#### 模块：database（数据库）

| 删除目录/文件 | 说明 |
|---|---|
| `src/modules/database/` | 前端：views/stores/api |
| `src-tauri/src/modules/database/` | 后端：commands/connection/models/repositories |

| 编辑文件 | 操作 |
|---|---|
| `src-tauri/src/modules/mod.rs` | 删除 `pub mod database;` |
| `src-tauri/src/lib.rs` | 删除 `database::commands::*` 对应的命令注册 |
| `src/core/router/index.ts` | 删除 `/data` 路由 |
| `src/layouts/DefaultLayout.vue` | 删除侧边栏中 database 相关菜单项 |

#### 模块：file_system（文件系统）

| 删除目录/文件 | 说明 |
|---|---|
| `src/modules/file_system/` | 前端：views/api |
| `src-tauri/src/modules/file_system/` | 后端：commands/handlers |

| 编辑文件 | 操作 |
|---|---|
| `src-tauri/src/modules/mod.rs` | 删除 `pub mod file_system;` |
| `src-tauri/src/lib.rs` | 删除 `file_system::commands::*` 对应的命令注册 |
| `src/core/router/index.ts` | 删除 `/files` 路由 |
| `src/layouts/DefaultLayout.vue` | 删除侧边栏中 file_system 相关菜单项 |

#### 模块：system（系统信息）

| 删除目录/文件 | 说明 |
|---|---|
| `src/modules/system/` | 前端：views/api |
| `src-tauri/src/modules/system/` | 后端：commands/info |

| 编辑文件 | 操作 |
|---|---|
| `src-tauri/src/modules/mod.rs` | 删除 `pub mod system;` |
| `src-tauri/src/lib.rs` | 删除 `system::commands::*` 对应的命令注册 |
| `src/core/router/index.ts` | 删除 `/system` 路由 |
| `src/layouts/DefaultLayout.vue` | 删除侧边栏中 system 相关菜单项 |

#### 可选工具：utils

如果你不需要某些工具函数，也可以局部删除：
- `src-tauri/src/utils/crypto.rs` — 加密工具（可单独删除）
- `src-tauri/src/utils/fs.rs` — 文件工具（可单独删除）
- 删除后更新 `src-tauri/src/utils/mod.rs`，移除对应 `pub mod xxx;`
- `src-tauri/src/utils/string.rs` 为核心工具，**保留**

## 路由

| 路径 | 页面 | 组件 |
|------|------|------|
| `/` | 首页 | `views/home/index.vue` |
| `/login` | 登录 | `modules/auth/views/Login.vue` |
| `/register` | 注册 | `modules/auth/views/Register.vue` |
| `/data` | 数据管理 | `modules/database/views/DataManager.vue` |
| `/files` | 文件浏览 | `modules/file_system/views/FileExplorer.vue` |
| `/system` | 系统信息 | `modules/system/views/SystemInfo.vue` |
| `/about` | 关于 | `views/about/index.vue` |

## 推荐 IDE

- [VS Code](https://code.visualstudio.com/)
  - [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar)
  - [Deno](https://marketplace.visualstudio.com/items?itemName=denoland.vscode-deno)
  - [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## License

MIT