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

/// 移动文件到目标目录（先复制，成功后删除源文件，走回收站）
pub fn move_to_dir<P: AsRef<Path>>(src: P, dst_dir: P) -> Result<String, String> {
  let src_path = src.as_ref();
  let dst_dir_path = dst_dir.as_ref();

  // 确保目标目录存在
  std::fs::create_dir_all(dst_dir_path).map_err(|e| format!("创建目标目录失败: {}", e))?;

  // 构造目标路径（同名）
  let file_name = src_path
    .file_name()
    .ok_or_else(|| "无法获取文件名".to_string())?;
  let dst_path = dst_dir_path.join(file_name);

  // 如果目标已存在，加数字后缀避免覆盖
  let final_dst = if dst_path.exists() {
    let stem = dst_path
      .file_stem()
      .and_then(|s| s.to_str())
      .unwrap_or("file");
    let ext = dst_path
      .extension()
      .and_then(|s| s.to_str())
      .map(|e| format!(".{}", e))
      .unwrap_or_default();
    let mut counter = 1;
    loop {
      let candidate = dst_dir_path.join(format!("{}_{}{}", stem, counter, ext));
      if !candidate.exists() {
        break candidate;
      }
      counter += 1;
    }
  } else {
    dst_path
  };

  // 先复制
  std::fs::copy(src_path, &final_dst).map_err(|e| format!("复制到目标目录失败: {}", e))?;
  // 成功后删除源文件（走回收站，可恢复）
  trash::delete(src_path).map_err(|e| format!("删除源文件失败: {}", e))?;

  final_dst
    .to_str()
    .map(|s| s.to_string())
    .ok_or_else(|| "目标路径包含非法字符".to_string())
}
