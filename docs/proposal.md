[200~以下是一个零前端构建依赖（纯静态 HTML/CSS/JS + Rust）的 **Tauri v2** 最小可运行项目模板，包含无边框透明胶囊气泡、鼠标拖拽移动、置顶显示以及整秒对齐的本地/北京双时钟。

### 目录结构

```text
clock-bubble/
├── src/
│   └── index.html                  # 气泡界面与双时钟逻辑
└── src-tauri/
    ├── Cargo.toml                  # Rust 依赖配置
    ├── tauri.conf.json             # 窗口透明、置顶与尺寸配置
    ├── capabilities/
    │   └── default.json            # Tauri v2 拖拽权限配置
    └── src/
        └── main.rs                 # 应用入口

```

---

### 1. 窗口配置 (`src-tauri/tauri.conf.json`)

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "clock-bubble",
  "version": "0.1.0",
  "identifier": "com.local.clock-bubble",
  "build": {
    "frontendDist": "../src"
  },
  "app": {
    "macOSPrivateApi": true,
    "windows": [
      {
        "label": "main",
        "title": "Clock Bubble",
        "width": 250,
        "height": 84,
        "decorations": false,
        "transparent": true,
        "alwaysOnTop": true,
        "skipTaskbar": true,
        "resizable": false,
        "shadow": false
      }
    ],
    "security": {
      "csp": "default-src 'self'; style-src 'self' 'unsafe-inline'; script-src 'self' 'unsafe-inline'"
    }
  }
}

```

### 2. 权限配置 (`src-tauri/capabilities/default.json`)

Tauri v2 默认收紧了窗口权限，必须显式授予 `core:window:allow-start-dragging`，前端的 `data-tauri-drag-region` 才能拖动窗口：

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Allow dragging the transparent clock bubble",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "core:window:allow-start-dragging"
  ]
}

```

### 3. Rust 后端 (`src-tauri/Cargo.toml` & `src-tauri/src/main.rs`)

**`src-tauri/Cargo.toml`**

```toml
[package]
name = "clock-bubble"
version = "0.1.0"
edition = "2021"

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = ["macos-private-api"] }

```

*(注：需同时在 `src-tauri/` 下保留标准的 `build.rs`，内容仅为 `fn main() { tauri_build::build() }`)*

**`src-tauri/src/main.rs`**

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // macOS 下隐藏 Dock 栏图标，使其作为纯桌面挂件运行
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

```

### 4. 前端气泡界面 (`src/index.html`)

使用 `font-variant-numeric: tabular-nums` 锁定等宽数字，避免每秒跳动时文字宽度抖动：

```html
<!DOCTYPE html>
<html lang="zh-CN">
<head>
  <meta charset="UTF-8" />
  <style>
    * {
      box-sizing: border-box;
      margin: 0;
      padding: 0;
      user-select: none;
      -webkit-user-select: none;
    }

    html, body {
      width: 100%;
      height: 100%;
      background: transparent;
      overflow: hidden;
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    }

    /* 留出 8px 外边距用于渲染柔和阴影，防止被窗口边缘裁切 */
    body {
      padding: 8px;
      display: flex;
      align-items: center;
      justify-content: center;
    }

    .bubble {
      width: 100%;
      height: 100%;
      background: rgba(24, 24, 27, 0.85);
      backdrop-filter: blur(12px);
      -webkit-backdrop-filter: blur(12px);
      border: 1px solid rgba(255, 255, 255, 0.14);
      border-radius: 999px;
      box-shadow: 0 4px 12px rgba(0, 0, 0, 0.28);
      display: flex;
      align-items: center;
      justify-content: space-evenly;
      padding: 0 16px;
      color: #f4f4f5;
      cursor: grab;
    }

    .bubble:active {
      cursor: grabbing;
    }

    .clock-block {
      display: flex;
      flex-direction: column;
      align-items: center;
      /* 确保子元素不拦截拖拽事件 */
      pointer-events: none;
    }

    .label {
      font-size: 10px;
      letter-spacing: 0.06em;
      color: #a1a1aa;
      margin-bottom: 2px;
    }

    .time {
      font-size: 16px;
      font-weight: 600;
      font-variant-numeric: tabular-nums;
      letter-spacing: 0.02em;
    }

    .divider {
      width: 1px;
      height: 28px;
      background: rgba(255, 255, 255, 0.15);
      pointer-events: none;
    }
  </style>
</head>
<body>
  <div class="bubble" data-tauri-drag-region>
    <div class="clock-block">
      <span class="label" id="local-label">LOCAL</span>
      <span class="time" id="local-time">--:--:--</span>
    </div>
    <div class="divider"></div>
    <div class="clock-block">
      <span class="label">BEIJING</span>
      <span class="time" id="beijing-time">--:--:--</span>
    </div>
  </div>

  <script>
    const localZone = Intl.DateTimeFormat().resolvedOptions().timeZone;
    // 提取城市名作为简短标签，例如 "America/New_York" -> "NEW YORK"
    const shortCity = localZone.split('/').pop().replace(/_/g, ' ').toUpperCase();
    document.getElementById('local-label').textContent = shortCity || 'LOCAL';

    const localFormatter = new Intl.DateTimeFormat('en-GB', {
      timeZone: localZone,
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
      hour12: false
    });

    const beijingFormatter = new Intl.DateTimeFormat('en-GB', {
      timeZone: 'Asia/Shanghai',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
      hour12: false
    });

    const localEl = document.getElementById('local-time');
    const beijingEl = document.getElementById('beijing-time');

    function updateClocks() {
      const now = new Date();
      localEl.textContent = localFormatter.format(now);
      beijingEl.textContent = beijingFormatter.format(now);

      // 对齐下一秒整点刷新
      const delay = 1000 - (now.getTime() % 1000);
      setTimeout(updateClocks, delay);
    }

    updateClocks();
  </script>
</body>
</html>

```

### 运行与构建

在项目根目录执行以下命令即可启动开发预览或打包成单文件可执行程序：

```bash
# 安装 Tauri CLI（如尚未安装）
cargo install tauri-cli --version "^2"

# 开发模式运行
cargo tauri dev

# 构建发布包
cargo tauri build

```
