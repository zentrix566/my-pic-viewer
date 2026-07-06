# 图标资源目录

Tauri 打包 (`npm run tauri:build`) 时必须存在下列图标文件，才能生成 exe：

- `32x32.png`
- `128x128.png`
- `128x128@2x.png`
- `icon.icns` (macOS)
- `icon.ico` (Windows)

## 一键生成

将一张 **1024x1024 的透明背景 PNG** 放到项目任意位置（例如 `src-tauri/icons/source.png`），
然后在项目根目录执行：

```bash
npx tauri icon src-tauri/icons/source.png
```

该命令会自动填充上述所有图标文件（默认输出到 `src-tauri/icons/`）。

## 开发模式

只运行 `npm run tauri:dev` 时可以暂时缺少 ico/icns，但仍推荐先生成一次以获得完整体验。
