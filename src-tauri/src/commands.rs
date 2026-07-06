use crate::{exif, file_ops, image_scan};
use serde::Serialize;
use std::path::{Path, PathBuf};
use tauri_plugin_clipboard_manager::ClipboardExt;

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

/// 把图片放到剪贴板。Tauri 剪贴板插件目前主要支持文本，我们退而写入文件路径文本
/// 让用户可以在其他程序里粘贴路径；后续可扩展成写入位图
#[tauri::command]
pub fn copy_image_to_clipboard(app: tauri::AppHandle, path: String) -> Result<(), String> {
  app.clipboard().write_text(path).map_err(|e| e.to_string())
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
