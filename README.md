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
- **关于对话框**：工具栏右上角 `?` 按钮查看软件信息（作者 zentrix566、版本、许可证）

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
| Ctrl+C | 复制图片到剪贴板 |
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

生成应用图标（如需替换默认占位图标，把 `src-tauri/icons/source.png` 换成你的 1024×1024 PNG 后重跑；仓库已附带一版占位图标）：

```bash
npx tauri icon src-tauri/icons/source.png
```

启动开发模式（会同时起 Vite 与 Tauri 窗口，热更新）：

```bash
npm run tauri:dev
```

构建绿色版 exe（产物在 `src-tauri/target/release/`）：

```bash
npm run tauri:build
```

构建完成后会得到两个产物：

- **绿色版单 exe（首选）**：`src-tauri/target/release/my-pic-viewer.exe` —— 直接拷到任何目录双击即可运行，无需安装
- **NSIS 安装包**：`src-tauri/target/release/bundle/nsis/my-pic-viewer_0.1.0_x64-setup.exe` —— 传统安装程序，双击安装到用户目录

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

## 发布新版本（GitHub Release）

本项目已配置 GitHub Actions（`.github/workflows/release.yml`），**打 tag 就自动构建 & 发布**：

```bash
# 1. 更新版本号（package.json / src-tauri/Cargo.toml / src-tauri/tauri.conf.json 三处保持一致）
# 2. 提交
git add -A
git commit -m "chore: 发布 v1.0.1"

# 3. 打 tag（tag 名必须以 v 开头）
git tag v1.0.1
git push origin main
git push origin v1.0.1
```

推 tag 后，GitHub Actions 会：

1. 在 Windows runner 上安装 Node + Rust
2. 跑 `npm ci` + `tauri build`
3. 自动创建 GitHub Release，附带以下产物：
   - `my-pic-viewer_<version>_portable.exe`（**绿色版单 exe，推荐**）
   - `my-pic-viewer_<version>_x64-setup.exe`（NSIS 安装包）
   - `my-pic-viewer_<version>_x64-setup.nsis.zip`（NSIS 归档）

整个过程约 5-10 分钟，进度可以在仓库 `Actions` 页面查看。

如果只想手动测一次 workflow（不打 tag、不发 release），可以在 GitHub 仓库 `Actions → Release → Run workflow` 手动触发。

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

- 构建产物 `target/release/my-pic-viewer.exe` 就是**单个 .exe**，直接拷到任何位置双击即用
- 已通过 `bundle.publisher` / `bundle.copyright` 把作者 zentrix566 写入 Windows exe 版本资源；右键 exe → 属性 → 详细信息 里能看到"公司"、"版权"、"文件说明"等字段
- 不写系统注册表；文件关联在 Windows"打开方式"里手动选择本程序一次即可
- 单实例通过 `tauri-plugin-single-instance` 实现，命令行参数会转发给已运行的窗口
- 用户配置默认写在 `%APPDATA%\com.zentrix566.mypicviewer`，如需真正便携可后续加"配置随 exe 走"选项

## 作者

zentrix566 · <zentrix566@gmail.com>

## 许可证

[MIT](./LICENSE)
