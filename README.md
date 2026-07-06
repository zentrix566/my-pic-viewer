# my-pic-viewer

> 一个真正绿色的 Windows 图片浏览器：单个 exe，双击即用，无需安装。

用 Tauri + Vue 3 + TypeScript 实现。产物 exe 约 5-10MB，WebView2 使用系统内置，无需捆绑运行时。

## 主要功能

- **左右浏览**：方向键 `←` `→` 或 `A` `D` 切换上一张 / 下一张
- **滚轮缩放**：以鼠标位置为中心缩放，`+` `-` 键盘缩放，`1` 回到 100%，`F` 或双击适应窗口
- **拖拽平移**：鼠标按住并拖动图像
- **旋转**：`R` 顺时针 90°，`Shift+R` 逆时针 90°
- **Delete 键删除**：安全地移动到系统回收站，可恢复；带二次确认弹窗
- **缩略图侧栏**：底部横向缩略图条，点击跳转；按 `T` 切换显示
- **EXIF 信息面板**：显示相机、镜头、光圈、快门、ISO、焦距、拍摄时间、GPS 等；按 `I` 切换显示
- **复制 / 另存为 / 重命名**：`Ctrl+C` / `Ctrl+S` / `F2`
- **全屏模式**：`F11` 切换，`Esc` 退出
- **文件关联**：可从资源管理器"打开方式"选择本程序，双击图片直接打开
- **单实例**：已开着窗口时再双击图片会切换到已有窗口，而不是开新的
- **拖拽打开**：直接把图片拖进窗口即可加载所在目录

支持格式：JPG / JPEG / PNG / GIF / BMP / WebP / ICO / TIF / TIFF

## 完整快捷键

| 按键 | 功能 |
|---|---|
| ← / → 或 A / D | 上一张 / 下一张 |
| Home / End | 首张 / 末张 |
| PgUp / PgDn | 跳 10 张 |
| 鼠标滚轮 | 以鼠标位置为中心缩放 |
| + / - | 键盘缩放 |
| 1 / 0 | 回到 100% |
| F / 双击 | 适应窗口 |
| R / Shift+R | 顺时针 / 逆时针旋转 |
| Delete | 删除到回收站 |
| Ctrl+O | 打开文件 |
| Ctrl+C | 复制当前路径 |
| Ctrl+S | 另存为 |
| F2 | 重命名 |
| I | 切换信息面板 |
| T | 切换缩略图条 |
| F11 | 全屏切换 |
| Esc | 退出全屏 |

## 环境要求

- **Node.js** 18+（推荐 20+）
- **Rust** 稳定版（`rustup` 一键安装：https://rustup.rs）
- **Windows 10 20H2+**（内置 WebView2；老系统需要装 [Evergreen Runtime](https://developer.microsoft.com/microsoft-edge/webview2/)）

## 运行方式

首次拉取代码后，安装前端依赖：

```bash
npm install
```

生成应用图标（首次构建前执行一次；把你的 logo 换成 1024×1024 的 PNG 后运行）：

```bash
npx tauri icon path/to/your-logo.png
```

启动开发模式（会同时起 Vite 与 Tauri 窗口，热更新）：

```bash
npm run tauri:dev
```

构建绿色版 exe（产物在 `src-tauri/target/release/`）：

```bash
npm run tauri:build
```

## 常用命令

仅构建前端产物（用于调试打包）：

```bash
npm run build
```

预览前端产物（不含 Tauri 壳）：

```bash
npm run preview
```

以 Vite dev server 单独运行前端（无 Tauri API 时会报错，仅用于调 UI）：

```bash
npm run dev
```

清理 Rust 构建缓存（占空间时）：

```bash
cargo clean --manifest-path src-tauri/Cargo.toml
```

## 目录结构

```
my-pic-viewer/
├── src/                     # Vue 前端源码
│   ├── components/          # UI 组件：Canvas / Thumbnail / Info / Toolbar / StatusBar
│   ├── composables/         # 组合式函数：图片列表、视口、快捷键、Tauri API
│   ├── App.vue              # 顶层布局与业务编排
│   └── main.ts              # Vue 入口
├── src-tauri/               # Rust 后端 + Tauri 配置
│   ├── src/
│   │   ├── main.rs          # 可执行入口
│   │   ├── lib.rs           # Tauri Builder / 插件 / 单实例
│   │   ├── commands.rs      # #[tauri::command] 汇总
│   │   ├── image_scan.rs    # 目录扫描 & 图片格式过滤
│   │   ├── exif.rs          # EXIF 解析
│   │   └── file_ops.rs      # 删除到回收站 / 重命名 / 拷贝
│   ├── capabilities/        # Tauri 权限
│   ├── icons/               # 应用图标（需先生成）
│   ├── Cargo.toml
│   └── tauri.conf.json
├── index.html
├── package.json
├── vite.config.ts
├── tsconfig.json
├── LICENSE                  # MIT
└── README.md
```

## 关于"绿色版"

- 构建产物是**单个 .exe**，直接拷到任何位置双击即用
- 不写系统注册表；文件关联在 Windows"打开方式"里手动选择本程序一次即可
- 单实例通过 `tauri-plugin-single-instance` 实现，命令行参数会转发给已运行的窗口
- 用户配置默认写在 `%APPDATA%\com.zentrix566.mypicviewer`，如需真正便携可后续加"配置随 exe 走"选项

## 作者

zentrix566 · <zentrix566@gmail.com>

## 许可证

[MIT](./LICENSE)
