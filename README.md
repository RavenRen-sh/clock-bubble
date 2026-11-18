# Clock Bubble (桌面双时钟悬浮气泡)

基于 **Tauri v2** 与纯静态 HTML/CSS/JS 的桌面置顶透明时钟胶囊气泡。

## 特性

- **轻量极简**：零前端构建依赖，纯原生 HTML/CSS/JavaScript + Rust。
- **透明悬浮**：无边框、无阴影裁切、背景高斯模糊（`backdrop-filter: blur(12px)`）、半透明磨砂质感。
- **全局置顶 & 拖拽**：固定在屏幕最上层（`alwaysOnTop`），支持在胶囊任意区域鼠标按住拖拽定位（`data-tauri-drag-region`）。
- **双时钟显示**：
  - 本地时区：自动解析当前系统所在城市名称并格式化时间（如 `LOS ANGELES`、`NEW YORK`、`SHANGHAI`）。
  - 北京时区：固定显示北京时间（`Asia/Shanghai`）。
- **精准对齐**：通过 `1000 - (now.getTime() % 1000)` 动态对齐每秒的整点翻转；采用 `tabular-nums` 等宽数字，跳秒不抖动。

## 目录结构

```text
.
├── docs/
│   └── proposal.md                 # 项目提案与设计规格说明
├── src/
│   └── index.html                  # 气泡界面与双时钟逻辑
└── src-tauri/
    ├── Cargo.toml                  # Rust 依赖配置
    ├── build.rs                    # Tauri 构建脚本
    ├── tauri.conf.json             # 窗口透明、置顶与尺寸配置
    ├── capabilities/
    │   └── default.json            # Tauri v2 拖拽与窗口权限配置
    └── src/
        └── main.rs                 # 应用入口
```

## 开发与运行

### 1. 前置依赖

- **Rust 工具链**：`rustc` 与 `cargo`
- **Tauri CLI**：
  ```bash
  cargo install tauri-cli --version "^2"
  ```
- **系统依赖（Linux 环境需安装 WebKitGTK 与 GTK3）**：
  ```bash
  # Ubuntu / Debian
  sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
  ```

### 2. 本地开发调试

在项目根目录下执行：

```bash
cargo tauri dev
```

或进入 `src-tauri` 目录：

```bash
cargo run
```

### 3. 构建发布包

```bash
cargo tauri build
```
构建产物将输出至 `src-tauri/target/release/`。

## 许可证

本项目基于 [MIT License](LICENSE) 开源。
