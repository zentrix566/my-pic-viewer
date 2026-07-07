<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { listen } from '@tauri-apps/api/event'
import { getName, getVersion } from '@tauri-apps/api/app'
import { confirm, message, open as openDialog, save as saveDialog } from '@tauri-apps/plugin-dialog'

import ImageCanvas from './components/ImageCanvas.vue'
import ThumbnailStrip from './components/ThumbnailStrip.vue'
import InfoPanel from './components/InfoPanel.vue'
import Toolbar from './components/Toolbar.vue'
import StatusBar from './components/StatusBar.vue'

import { useImageList } from './composables/useImageList'
import { useViewport } from './composables/useViewport'
import { useKeyboard } from './composables/useKeyboard'
import { tauriApi } from './composables/useTauriApi'
import type { ExifInfo, ImageFileInfo } from './types'

// 图片列表
const {
  items,
  currentIndex,
  currentPath,
  total,
  hasImage,
  openPath: openPathInList,
  prev,
  next,
  first,
  last,
  jump,
  goTo,
  removeCurrent,
  updateCurrentPath
} = useImageList()

// 视口
const {
  scale,
  fitToWindow,
  actualSize,
  zoomIn,
  zoomOut,
  rotateBy
} = useViewport()

// UI 显示开关
const showInfo = ref(true)
const showThumbnails = ref(true)
const isFullscreen = ref(false)

// 当前图片元数据
const fileInfo = ref<ImageFileInfo | null>(null)
const exif = ref<ExifInfo | null>(null)
const naturalWidth = ref(0)
const naturalHeight = ref(0)

const currentName = computed(() => fileInfo.value?.name ?? '')

// ---------- 打开文件 ----------
const supportedFilters = [
  {
    name: '图片',
    extensions: ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp', 'ico', 'tif', 'tiff']
  }
]

async function chooseFile() {
  const picked = await openDialog({
    multiple: false,
    directory: false,
    filters: supportedFilters
  })
  if (typeof picked === 'string' && picked) {
    await openPath(picked)
  }
}

async function openPath(path: string) {
  try {
    await openPathInList(path)
  } catch (err) {
    await message(`打开失败: ${err}`, { title: '错误', kind: 'error' })
  }
}

// ---------- 换图时刷新元数据 ----------
watch(
  currentPath,
  async (p) => {
    if (!p) {
      fileInfo.value = null
      exif.value = null
      return
    }
    try {
      fileInfo.value = await tauriApi.readImageInfo(p)
    } catch {
      fileInfo.value = null
    }
    try {
      exif.value = await tauriApi.readExifInfo(p)
    } catch {
      exif.value = null
    }
  },
  { immediate: true }
)

function onImageLoaded(payload: { width: number; height: number }) {
  naturalWidth.value = payload.width
  naturalHeight.value = payload.height
}

// ---------- 删除到回收站 ----------
async function deleteCurrent() {
  const p = currentPath.value
  if (!p) return
  const ok = await confirm(`确定要将「${fileInfo.value?.name ?? p}」移动到回收站吗？`, {
    title: '删除确认',
    kind: 'warning'
  })
  if (!ok) return
  try {
    await tauriApi.deleteToTrash(p)
    removeCurrent()
  } catch (err) {
    await message(`删除失败: ${err}`, { title: '错误', kind: 'error' })
  }
}

// ---------- 复制路径 ----------
async function copyCurrent() {
  const p = currentPath.value
  if (!p) return
  try {
    await tauriApi.copyImageToClipboard(p)
  } catch (err) {
    await message(`复制失败: ${err}`, { title: '错误', kind: 'error' })
  }
}

// ---------- 另存为 ----------
async function saveAsCurrent() {
  const p = currentPath.value
  if (!p) return
  const dst = await saveDialog({
    defaultPath: fileInfo.value?.name,
    filters: supportedFilters
  })
  if (typeof dst !== 'string' || !dst) return
  try {
    await tauriApi.copyFileTo(p, dst)
  } catch (err) {
    await message(`另存为失败: ${err}`, { title: '错误', kind: 'error' })
  }
}

// ---------- 重命名 ----------
async function renameCurrent() {
  const p = currentPath.value
  if (!p || !fileInfo.value) return
  const oldName = fileInfo.value.name
  const input = window.prompt('新文件名（含扩展名）:', oldName)
  if (!input || input === oldName) return
  try {
    const newPath = await tauriApi.renameFile(p, input)
    updateCurrentPath(newPath)
    fileInfo.value = await tauriApi.readImageInfo(newPath)
  } catch (err) {
    await message(`重命名失败: ${err}`, { title: '错误', kind: 'error' })
  }
}

// ---------- 全屏 ----------
async function toggleFullscreen() {
  const win = getCurrentWebviewWindow()
  const cur = await win.isFullscreen()
  await win.setFullscreen(!cur)
  isFullscreen.value = !cur
}

async function exitFullscreen() {
  if (!isFullscreen.value) return
  const win = getCurrentWebviewWindow()
  await win.setFullscreen(false)
  isFullscreen.value = false
}

// ---------- 关于对话框 ----------
async function showAbout() {
  let appName = 'my-pic-viewer'
  let appVersion = '0.1.0'
  try {
    appName = await getName()
    appVersion = await getVersion()
  } catch {
    /* 开发时若未就绪，用默认值 */
  }
  const body = [
    `${appName} v${appVersion}`,
    '',
    '一个绿色版图片浏览器',
    '',
    '作者：zentrix566',
    '邮箱：zentrix566@gmail.com',
    '项目：https://github.com/zentrix566/my-pic-viewer',
    '许可证：MIT'
  ].join('\n')
  await message(body, { title: '关于 my-pic-viewer', kind: 'info' })
}

// ---------- 键盘 ----------
useKeyboard({
  onPrev: () => prev(),
  onNext: () => next(),
  onFirst: () => first(),
  onLast: () => last(),
  onJump: (offset: number) => jump(offset),
  onZoomIn: () => zoomIn(),
  onZoomOut: () => zoomOut(),
  onActualSize: () => actualSize(),
  onFit: () => fitToWindow(),
  onRotateCW: () => rotateBy(90),
  onRotateCCW: () => rotateBy(-90),
  onDelete: () => void deleteCurrent(),
  onOpen: () => void chooseFile(),
  onCopy: () => void copyCurrent(),
  onSaveAs: () => void saveAsCurrent(),
  onRename: () => void renameCurrent(),
  onToggleInfo: () => (showInfo.value = !showInfo.value),
  onToggleThumbnails: () => (showThumbnails.value = !showThumbnails.value),
  onToggleFullscreen: () => void toggleFullscreen(),
  onExitFullscreen: () => void exitFullscreen()
})

// ---------- 拖拽文件 & 命令行参数 ----------
onMounted(async () => {
  await listen<string>('open-file', (e) => {
    if (e.payload) void openPath(e.payload)
  })

  const win = getCurrentWebviewWindow()
  await win.onDragDropEvent(async (event) => {
    if (event.payload.type === 'drop') {
      const paths = event.payload.paths
      if (paths && paths[0]) await openPath(paths[0])
    }
  })
})
</script>

<template>
  <div class="app">
    <Toolbar
      :has-image="hasImage"
      :show-info="showInfo"
      :show-thumbnails="showThumbnails"
      @open="chooseFile"
      @prev="prev"
      @next="next"
      @zoom-in="zoomIn"
      @zoom-out="zoomOut"
      @fit="fitToWindow"
      @actual-size="actualSize"
      @rotate-cw="rotateBy(90)"
      @rotate-ccw="rotateBy(-90)"
      @delete="deleteCurrent"
      @copy="copyCurrent"
      @save-as="saveAsCurrent"
      @rename="renameCurrent"
      @toggle-info="showInfo = !showInfo"
      @toggle-thumbnails="showThumbnails = !showThumbnails"
      @toggle-fullscreen="toggleFullscreen"
      @about="showAbout"
    />

    <div class="body">
      <ImageCanvas
        :path="currentPath"
        class="canvas"
        @dblclick="fitToWindow"
        @loaded="onImageLoaded"
      />
      <InfoPanel
        v-if="showInfo"
        :file-info="fileInfo"
        :exif="exif"
        :width="naturalWidth"
        :height="naturalHeight"
      />
    </div>

    <ThumbnailStrip
      v-if="showThumbnails"
      :items="items"
      :current-index="currentIndex"
      @select="(i: number) => goTo(i)"
    />

    <StatusBar
      :index="currentIndex"
      :total="total"
      :scale="scale"
      :width="naturalWidth"
      :height="naturalHeight"
      :file-name="currentName"
    />
  </div>
</template>

<style scoped>
.app {
  display: flex;
  flex-direction: column;
  width: 100vw;
  height: 100vh;
  background: #0d0d10;
  color: #e0e0e8;
  overflow: hidden;
}
.body {
  flex: 1;
  min-height: 0;
  display: flex;
}
.canvas {
  flex: 1;
  min-width: 0;
}
</style>
