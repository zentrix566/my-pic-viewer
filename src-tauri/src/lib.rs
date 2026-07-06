use tauri::{Emitter, Manager};

mod commands;
mod exif;
mod file_ops;
mod image_scan;

/// 应用入口，被 main.rs 调用
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    // 单实例：第二次启动时，把参数发给已有实例，避免重复开窗口
    .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
      // args[0] 是可执行文件路径，从 1 开始才是真正传入的文件参数
      let file_arg = args.into_iter().skip(1).find(|a| !a.starts_with('-'));
      if let Some(path) = file_arg {
        let _ = app.emit("open-file", path);
      }
      if let Some(win) = app.get_webview_window("main") {
        let _ = win.unminimize();
        let _ = win.set_focus();
      }
    }))
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_clipboard_manager::init())
    .setup(|app| {
      // 首次启动时，从 CLI 参数里取要打开的文件（Windows 文件关联双击时携带）
      let args: Vec<String> = std::env::args().skip(1).collect();
      let initial_file = args.into_iter().find(|a| !a.starts_with('-'));
      if let Some(path) = initial_file {
        // 延迟到前端 ready 后再发，用一次性 setTimeout 前端里再监听
        let handle = app.handle().clone();
        std::thread::spawn(move || {
          std::thread::sleep(std::time::Duration::from_millis(400));
          let _ = handle.emit("open-file", path);
        });
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::list_images_in_dir,
      commands::read_image_info,
      commands::read_exif_info,
      commands::delete_to_trash,
      commands::rename_file,
      commands::copy_file_to,
      commands::copy_image_to_clipboard,
    ])
    .run(tauri::generate_context!())
    .expect("启动 Tauri 应用失败");
}
