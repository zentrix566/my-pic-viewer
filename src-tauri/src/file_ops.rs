use std::path::{Path, PathBuf};

/// 删除到系统回收站（可恢复）
pub fn delete_to_recycle_bin<P: AsRef<Path>>(path: P) -> Result<(), String> {
  trash::delete(path.as_ref()).map_err(|e| e.to_string())
}

/// 重命名文件（同目录）
pub fn rename<P: AsRef<Path>>(old: P, new_name: &str) -> Result<String, String> {
  let old_path: &Path = old.as_ref();
  let parent = old_path
    .parent()
    .ok_or_else(|| "无法获取父目录".to_string())?;
  let new_path: PathBuf = parent.join(new_name);
  if new_path.exists() {
    return Err(format!("目标文件已存在: {}", new_path.display()));
  }
  std::fs::rename(old_path, &new_path).map_err(|e| e.to_string())?;
  new_path
    .to_str()
    .map(|s| s.to_string())
    .ok_or_else(|| "路径包含非法字符".to_string())
}

/// 拷贝文件到指定路径（用于「另存为」）
pub fn copy_to<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<(), String> {
  std::fs::copy(src.as_ref(), dst.as_ref())
    .map(|_| ())
    .map_err(|e| e.to_string())
}
