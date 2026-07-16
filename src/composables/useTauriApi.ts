// 封装所有对 Rust 后端 command 的调用
import { invoke } from '@tauri-apps/api/core'
import type { ExifInfo, ImageFileInfo, ImageListResult, UpdateCheckResult } from '../types'

export const tauriApi = {
  listImagesInDir(path: string) {
    return invoke<ImageListResult>('list_images_in_dir', { path })
  },
  readImageInfo(path: string) {
    return invoke<ImageFileInfo>('read_image_info', { path })
  },
  readExifInfo(path: string) {
    return invoke<ExifInfo>('read_exif_info', { path })
  },
  deleteToTrash(path: string) {
    return invoke<void>('delete_to_trash', { path })
  },
  renameFile(path: string, newName: string) {
    return invoke<string>('rename_file', { path, newName })
  },
  copyFileTo(src: string, dst: string) {
    return invoke<void>('copy_file_to', { src, dst })
  },
  copyImageToClipboard(path: string) {
    return invoke<void>('copy_image_to_clipboard', { path })
  },
  takePendingFile() {
    return invoke<string | null>('take_pending_file')
  },
  checkUpdate() {
    return invoke<UpdateCheckResult>('check_update')
  },
  openUrl(url: string) {
    return invoke<void>('open_url', { url })
  },
  moveFileToDir(src: string, dstDir: string) {
    return invoke<string>('move_file_to_dir', { src, dstDir })
  }
}
