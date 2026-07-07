use std::path::Path;
use std::time::UNIX_EPOCH;

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

/// 取文件的创建时间（ms since UNIX_EPOCH）；若平台不支持则退回修改时间；
/// 都拿不到则返回 0，保证排序稳定不 panic
fn file_created_ms(path: &Path) -> i128 {
  let meta = match std::fs::metadata(path) {
    Ok(m) => m,
    Err(_) => return 0,
  };
  let t = meta.created().or_else(|_| meta.modified());
  match t {
    Ok(t) => match t.duration_since(UNIX_EPOCH) {
      Ok(d) => d.as_millis() as i128,
      Err(e) => -(e.duration().as_millis() as i128), // 早于 1970 的极端情况
    },
    Err(_) => 0,
  }
}

/// 扫描目录下所有支持的图片，按文件创建时间倒序排序（新 → 旧）；
/// 若创建时间相同则按文件名自然序作为稳定的次级排序
pub fn list_directory_images<P: AsRef<Path>>(dir: P) -> Result<Vec<String>, String> {
  // 先收集 (路径字符串, 创建时间ms)，一次性 stat，避免排序里重复 IO
  let mut items: Vec<(String, i128)> = Vec::new();
  let read = std::fs::read_dir(dir.as_ref()).map_err(|e| e.to_string())?;
  for entry in read.flatten() {
    let path = entry.path();
    if path.is_file() && is_supported_image(&path) {
      if let Some(s) = path.to_str() {
        let ctime = file_created_ms(&path);
        items.push((s.to_string(), ctime));
      }
    }
  }
  // 创建时间倒序：新 → 旧；相同创建时间时按文件名自然升序，避免顺序抖动
  items.sort_by(|a, b| {
    b.1
      .cmp(&a.1)
      .then_with(|| natord::compare_ignore_case(&a.0, &b.0))
  });
  Ok(items.into_iter().map(|(p, _)| p).collect())
}
