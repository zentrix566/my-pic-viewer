// 后端 Rust 返回值的对应类型定义

export interface ImageListResult {
  items: string[]
  currentIndex: number
}

export interface ImageFileInfo {
  path: string
  name: string
  dir: string
  sizeBytes: number
  modifiedMs: number | null
}

export interface ExifInfo {
  cameraMake?: string | null
  cameraModel?: string | null
  lensModel?: string | null
  aperture?: string | null
  shutterSpeed?: string | null
  iso?: string | null
  focalLength?: string | null
  datetimeOriginal?: string | null
  gpsLatitude?: string | null
  gpsLongitude?: string | null
  orientation?: string | null
}

// 前端 UI 状态用的图片项
export interface ImageItem {
  path: string
  name: string
}

// 检查更新的返回结果
export interface UpdateCheckResult {
  hasUpdate: boolean
  latestTag: string | null
  currentVersion: string
  releaseUrl: string | null
  error: string | null
}
