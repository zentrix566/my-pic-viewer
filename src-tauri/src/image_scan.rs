use std::path::Path;

/// 支持的图片扩展名（小写）
pub const SUPPORTED_EXTS: &[&str] = &[
  "jpg", "jpeg", "png", "gif", "bmp", "webp", "ico", "tif", "tiff",
];

/// 判断某个文件名是否是支持的图片格式
pub fn is_supported_image<P: AsRef<Path>>(path: P) -> bool {
  match path.as_ref().extension().and_then(|e| e.to_str()) {
    Some(ext) => {
      let lower = ext.to_ascii_lowercase();
      SUPPORTED_EXTS.iter().any(|e| *e == lower)
    }
    None => false,
  }
}

/// 扫描目录下所有支持的图片，按自然顺序排序
pub fn list_directory_images<P: AsRef<Path>>(dir: P) -> Result<Vec<String>, String> {
  let mut items: Vec<String> = Vec::new();
  let read = std::fs::read_dir(dir.as_ref()).map_err(|e| e.to_string())?;
  for entry in read.flatten() {
    let path = entry.path();
    if path.is_file() && is_supported_image(&path) {
      if let Some(s) = path.to_str() {
        items.push(s.to_string());
      }
    }
  }
  // 自然排序：img2.jpg 在 img10.jpg 前面
  items.sort_by(|a, b| natord::compare_ignore_case(a, b));
  Ok(items)
}
