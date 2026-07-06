<script setup lang="ts">
import { computed } from 'vue'
import type { ExifInfo, ImageFileInfo } from '../types'

const props = defineProps<{
  fileInfo: ImageFileInfo | null
  exif: ExifInfo | null
  width: number
  height: number
}>()

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / 1024 / 1024).toFixed(2)} MB`
  return `${(bytes / 1024 / 1024 / 1024).toFixed(2)} GB`
}

function formatTime(ms: number | null | undefined): string {
  if (!ms) return '-'
  const d = new Date(ms)
  const pad = (n: number) => String(n).padStart(2, '0')
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())} ${pad(d.getHours())}:${pad(d.getMinutes())}:${pad(d.getSeconds())}`
}

const rows = computed(() => {
  const info = props.fileInfo
  const exif = props.exif
  const list: { label: string; value: string }[] = []
  if (info) {
    list.push({ label: '文件名', value: info.name })
    list.push({ label: '路径', value: info.dir })
    list.push({ label: '大小', value: formatSize(info.sizeBytes) })
    list.push({ label: '修改时间', value: formatTime(info.modifiedMs) })
  }
  if (props.width && props.height) {
    list.push({ label: '分辨率', value: `${props.width} × ${props.height}` })
  }
  if (exif) {
    const pushIfHas = (label: string, v?: string | null) => {
      if (v && v.trim()) list.push({ label, value: v })
    }
    pushIfHas('相机', [exif.cameraMake, exif.cameraModel].filter(Boolean).join(' '))
    pushIfHas('镜头', exif.lensModel)
    pushIfHas('光圈', exif.aperture)
    pushIfHas('快门', exif.shutterSpeed)
    pushIfHas('ISO', exif.iso)
    pushIfHas('焦距', exif.focalLength)
    pushIfHas('拍摄时间', exif.datetimeOriginal)
    pushIfHas('GPS 纬度', exif.gpsLatitude)
    pushIfHas('GPS 经度', exif.gpsLongitude)
    pushIfHas('方向', exif.orientation)
  }
  return list
})
</script>

<template>
  <aside class="info-panel">
    <h3 class="title">图像信息</h3>
    <div class="table">
      <div v-for="(row, i) in rows" :key="i" class="row">
        <div class="label">{{ row.label }}</div>
        <div class="value" :title="row.value">{{ row.value }}</div>
      </div>
      <div v-if="rows.length === 0" class="empty">无信息</div>
    </div>
  </aside>
</template>

<style scoped>
.info-panel {
  width: 260px;
  background: #17171c;
  border-left: 1px solid #2a2a30;
  color: #d0d0d8;
  overflow-y: auto;
  padding: 12px 14px;
  font-size: 12px;
}
.title {
  margin: 0 0 10px 0;
  font-size: 13px;
  color: #b0b0b8;
  font-weight: 600;
}
.table {
  display: flex;
  flex-direction: column;
  gap: 6px;
}
.row {
  display: grid;
  grid-template-columns: 68px 1fr;
  gap: 8px;
  line-height: 1.4;
}
.label {
  color: #7a7a86;
}
.value {
  word-break: break-all;
  color: #d8d8e0;
}
.empty {
  color: #6a6a75;
}
</style>
