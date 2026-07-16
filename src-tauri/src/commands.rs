use crate::{exif, file_ops, image_scan, update};
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::State;
use tauri_plugin_clipboard_manager::ClipboardExt;

/// 启动时被双击打开的文件路径，存于此供前端挂载后拉取（见 take_pending_file）
pub struct PendingFile(pub Mutex<Option<String>>);

/// 图片列表返回值
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageListResult {
  pub items: Vec<String>,
  pub current_index: usize,
}

/// 图片文件基本信息
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageFileInfo {
  pub path: String,
  pub name: String,
  pub dir: String,
  pub size_bytes: u64,
  pub modified_ms: Option<i64>,
}

/// 扫描给定文件所在目录，返回同目录所有图片和当前文件的索引
#[tauri::command]
pub fn list_images_in_dir(path: String) -> Result<ImageListResult, String> {
  let p = PathBuf::from(&path);
  let dir = if p.is_dir() {
    p.clone()
  } else {
    p.parent()
      .ok_or_else(|| "无法获取父目录".to_string())?
      .to_path_buf()
  };
  let items = image_scan::list_directory_images(&dir)?;
  let normalized = normalize_path(&p);
  let current_index = items
    .iter()
    .position(|s| normalize_path(Path::new(s)) == normalized)
    .unwrap_or(0);
  Ok(ImageListResult {
    items,
    current_index,
  })
}

/// 读取图片文件的基本信息
#[tauri::command]
pub fn read_image_info(path: String) -> Result<ImageFileInfo, String> {
  let p = PathBuf::from(&path);
  let meta = std::fs::metadata(&p).map_err(|e| e.to_string())?;
  let modified_ms = meta
    .modified()
    .ok()
    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
    .map(|d| d.as_millis() as i64);
  Ok(ImageFileInfo {
    path: p.to_string_lossy().to_string(),
    name: p
      .file_name()
      .map(|s| s.to_string_lossy().to_string())
      .unwrap_or_default(),
    dir: p
      .parent()
      .map(|s| s.to_string_lossy().to_string())
      .unwrap_or_default(),
    size_bytes: meta.len(),
    modified_ms,
  })
}

/// 读取 EXIF 信息
#[tauri::command]
pub fn read_exif_info(path: String) -> Result<exif::ExifInfo, String> {
  exif::read_exif(&path)
}

/// 删除到回收站
#[tauri::command]
pub fn delete_to_trash(path: String) -> Result<(), String> {
  file_ops::delete_to_recycle_bin(&path)
}

/// 重命名文件；返回新的完整路径
#[tauri::command]
pub fn rename_file(path: String, new_name: String) -> Result<String, String> {
  file_ops::rename(&path, &new_name)
}

/// 复制文件到指定目标（另存为）
#[tauri::command]
pub fn copy_file_to(src: String, dst: String) -> Result<(), String> {
  file_ops::copy_to(&src, &dst)
}

/// 把图片本身（位图）放到剪贴板，而不是复制文件路径文本。
/// 用 image 解码任意常见格式为 RGBA，再交给剪贴板插件写入系统剪贴板。
#[tauri::command]
pub fn copy_image_to_clipboard(app: tauri::AppHandle, path: String) -> Result<(), String> {
  let rgba = image::open(&path)
    .map_err(|e| format!("无法解码图片: {e}"))?
    .to_rgba8();
  let (width, height) = (rgba.width(), rgba.height());
  let image = tauri::image::Image::new_owned(rgba.into_raw(), width, height);
  app
    .clipboard()
    .write_image(&image)
    .map_err(|e| e.to_string())
}

/// 取出并清空「启动时待打开的文件」，仅首次有效。
/// 前端完成挂载、监听就绪后调用，避免事件早于监听者发出而丢失。
#[tauri::command]
pub fn take_pending_file(state: State<PendingFile>) -> Option<String> {
  state.0.lock().unwrap().take()
}

/// Windows 上路径大小写不敏感，做归一化再比较
fn normalize_path(p: &Path) -> String {
  let s = p.to_string_lossy().replace('\\', "/");
  if cfg!(windows) {
    s.to_ascii_lowercase()
  } else {
    s
  }
}

/// 检查 GitHub 上是否有新版本
#[tauri::command]
pub fn check_update() -> update::UpdateCheckResult {
  update::check_latest_release()
}

/// 在系统默认浏览器中打开链接
#[tauri::command]
pub fn open_url(url: String) -> Result<(), String> {
  webbrowser::open(&url).map_err(|e| e.to_string())
}

/// 移动图片到指定分类目录（复制后删除原文件）
#[tauri::command]
pub fn move_file_to_dir(src: String, dst_dir: String) -> Result<String, String> {
  file_ops::move_to_dir(&src, &dst_dir)
}
