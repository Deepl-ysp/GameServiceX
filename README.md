
# GameServiceX - 视觉小说引擎 / Visual Novel Engine

# 目前还在开发中敬请期待 / Currently under development, stay tuned

---

## 项目简介 / Project Introduction

**GameServiceX**（简称 **GSX**）是一个基于 Tauri v2、Vue 3 和 TypeScript 构建的现代化轻量级视觉小说引擎。它利用 Rust 的高性能安全性处理后端逻辑，同时使用 Vue 3 组合式 API 提供流畅的响应式用户界面，支持视觉小说剧情展示、关卡选择、游戏设置、角色立绘和音频播放等功能。

GameServiceX is a modern, lightweight visual novel engine built with Tauri v2, Vue 3, and TypeScript. It leverages the high performance and security of Rust for backend logic, while using Vue 3 Composition API to provide a smooth, responsive user interface. It supports visual novel story presentation, level selection, game settings, character sprites, and audio playback features.

---

## 技术栈与依赖 / Tech Stack & Dependencies

### 前端 / Frontend
- **Vue.js** (^3.5.13): 渐进式 JavaScript 框架，使用组合式 API / Progressive JavaScript framework using Composition API
- **TypeScript** (~5.6.2): 类型安全的 JavaScript 超集 / Type-safe superset of JavaScript
- **Vue Router** (^5.0.6): Vue.js 官方路由管理器 / Official router manager for Vue.js
- **Vite** (^6.0.3): 下一代前端开发与构建工具 / Next-generation frontend development and build tool
- **Sass** (^1.99.0): CSS 预处理器 / CSS preprocessor

### 后端 (Rust) / Backend (Rust)
- **Tauri** (v2): 用于构建微小、快速且安全的桌面应用框架 / Framework for building tiny, fast, and secure desktop apps
- **Serde** (v1): Rust 序列化框架 / Rust serialization framework
- **Serde JSON** (v1): JSON 数据处理库 / JSON data processing library
- **Rodio** (0.19): 音频播放库 / Audio playback library
- **Base64** (0.22.1): Base64 编码/解码库 / Base64 encoding/decoding library

### Tauri 插件 / Tauri Plugins
- **plugin-opener** (^2): 用于打开 URL 和本地文件 / For opening URLs and local files
- **plugin-fs** (2.5.1): 提供文件系统访问能力 / Provides file system access
- **plugin-process** (2.3.1): 提供进程相关信息和管理能力 / Provides process information and management capabilities

---

## 项目结构 / Project Structure

```
ProjectRoot/                        // 项目根目录
├── src/                            // 前端代码目录
│   ├── components/                 // 前端组件目录
│   │   └── BaseButton.vue          // 预设通用按钮
│   ├── pages/                      // 前端页面目录
│   │   └── Home.vue                // 首页文件
│   ├── router/                     // 前端路由配置目录
│   │   └── index.ts                // 前端路由配置文件
│   ├── styles/                     // 通用样式文件
│   ├── utils/                      // 前端工具函数文件目录
│   │   └── image.ts                // 前端图片模块文件
│   ├── Main.vue                    // 前端挂载文件
│   └── main.ts                     // 前端入口文件
├── src-tauri/                      // 后端目录
│   ├── capabilities/               // 后端权限控制目录
│   ├── gen/                        // 安卓打包工具目录
│   ├── icons/                      // 应用图标目录
│   ├── src/                        // 后端代码目录
│   │   ├── cmd/                    // 后端接口指令目录
│   │   │   ├── image.rs            // 后端图片模块
│   │   │   └── mod.rs              // 导出模块内命名空间文件
│   │   ├── utils/                  // 后端工具模块目录
│   │   │   ├── fileAPI.rs          // 后端文件模块
│   │   │   └── mod.rs              // 导出模块内命名空间文件
│   │   ├── lib.rs                  // 后端库文件
│   │   └── main.rs                 // 后端入口文件
│   ├── user/                       // 用户文件夹
│   │   └── config.json             // 用户配置文件
│   ├── build.rs                    // 打包文件
│   ├── Cargo.lock                  // 后端依赖详细清单
│   ├── Cargo.toml                  // 后端依赖清单
│   └── tauri.conf.json             // 软件文章文件
├── index.html                      // 前端页面主节点挂载文件
├── package-lock.json               // 前端一依赖详细清单
├── package.json                    // 前端配置文件
├── README.md                       // 项目说明文件(本文件)
├── tsconfig.json                   // TypeScript配置文件
└── vite.config.ts                  // Vite配置文件
```

---

## 主要功能 / Main Features

1. **视觉小说剧情展示 / Visual Novel Story Presentation**
   - 支持自定义背景图片切换 / Supports custom background image switching
   - 支持角色立绘显示和位置调整 / Supports character sprite display and position adjustment
   - 支持对话系统 / Supports dialogue system

2. **关卡系统 / Level System**
   - JSON 格式的关卡数据 / JSON-formatted level data
   - 关卡选择界面 / Level selection interface
   - 分支剧情支持 / Branching story support

3. **音频系统 / Audio System**
   - 背景音乐（BGM）循环播放 / Background music (BGM) loop playback
   - 对话语音播放 / Dialogue voice playback
   - 音效（SFX）播放 / Sound effect (SFX) playback
   - 音量控制 / Volume control

4. **设置系统 / Settings System**
   - 窗口设置（大小、全屏） / Window settings (size, fullscreen)
   - 游戏设置（自动播放、自动保存） / Game settings (auto-play, auto-save)
   - 音频设置（音量、开关） / Audio settings (volume, toggles)
   - 文本设置（速度、大小、位置、对齐） / Text settings (speed, size, position, alignment)

5. **用户界面 / User Interface**
   - 现代化的主菜单 / Modern main menu
   - 响应式设计 / Responsive design
   - 流畅的动画效果 / Smooth animation effects

---

## 关卡数据格式 / Level Data Format

关卡数据使用 JSON 格式存储，支持以下类型：

Level data is stored in JSON format and supports the following types:

### 关卡列表 (LevelList.json)
```json
[
  {
    "levelID": "1",
    "levelName": "Level 1",
    "levelTitle": "Abydos Council Room",
    "levelBgm": "audio/bgm/theme_06.ogg",
    "levelBack": "images/background/bg_abydoscouncilroom.jpg",
    "levelStandingIllustration": "images/Character/Abydos/CountermeasuresCommittee/ShirokoSunaookami/01.webp",
    "levelFile": "levels/level_1.json",
    "sideQuest": [...]
  }
]
```
| **键 / Key**                   | **值 / Value** | **用途 / Use** |
|-------------------------------|---------------|--------------|
| **levelID**                   | 数字 / Number   | 用于关卡查询与解锁关卡  |
| **levelName**                 | 字符串 / String  | 关卡名称         |
| **levelTitle**                | 字符串 / String  | 关卡标题         |
| **levelBgm**                  | 字符串 / String  | 关卡初始背景音乐     |
| **levelBack**                 | 字符串 / String  | 关卡初始背景图片     |
| **levelStandingIllustration** | 字符串 / String  | 关卡初始角色立绘     |
| **levelFile**                 | 字符串 / String  | 关卡配置文件       |
| **sideQuest**                 | 对象 / Object   | 支线关卡配置       |

### 关卡剧情 (level_1.json)
```json
[
  {
    "type": "SetBack",
    "id": 1,
    "next": 2,
    "back": "images/background/example.jpg"
  },
  {
    "type": "SetCharacter",
    "id": 2,
    "next": 3,
    "character": "HoshinoTakanashi",
    "position": [1, 0, 0],
    "illustrationNumber": 1
  },
  {
    "type": "PlayBgm",
    "id": 3,
    "next": 4,
    "src": "audio/bgm/theme_01.ogg",
    "volume": 0.8
  },
  {
    "type": "Dialogue",
    "id": 4,
    "next": 5,
    "text": "这是一句对话。",
    "character": "HoshinoTakanashi",
    "vocSrc": "audio/voice/example.ogg"
  }
]
```
  
---

## 推荐 IDE 设置 / Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/)
- [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar)
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

---

## 快速开始 / Quick Start

### 前置要求 / Prerequisites

- **Node.js** (版本 18 或更高 / Version 18 or higher)
- **Rust** (使用 rustup 安装 / Install using rustup)
- **Tauri CLI** (通过 npm 安装 / Install via npm)

### 安装与运行 / Installation & Running

1. 安装前端依赖 / Install frontend dependencies：
```bash
npm install
```

2. 启动开发服务器 / Start development server：
```bash
npm run tauri dev
```

3. 构建发行版 / Build release version：
```bash
npm run tauri build
```

### 其他命令 / Other Commands

- 只启动前端开发服务器 / Start only frontend dev server：
```bash
npm run dev
```

- 构建前端 / Build frontend：
```bash
npm run build
```

---

## 贡献指南 / Contributing

本项目使用 **MIT 许可证**，我们欢迎任何形式的贡献（包括但不限于提交 Issue、Pull Request、功能建议等）。  
在贡献前，请确保你已阅读并理解 [LICENSE.md](LICENSE.md) 中的条款。

We welcome all forms of contributions (including but not limited to issues, pull requests, feature suggestions, etc.) under the **MIT License**.  
Before contributing, please ensure you have read and understood the terms in [LICENSE.md](LICENSE.md).

---

## 许可证 / License

本项目采用 **MIT 许可证**。  
This project is licensed under the **MIT License**.

```
MIT License

Copyright (c) [year] [fullname]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

完整的许可证文本请参阅 [LICENSE.md](LICENSE.md) 文件。  
For the full license text, please refer to the [LICENSE.md](LICENSE.md) file.