<script setup lang="ts">
defineProps<{
  hasImage: boolean
  showInfo: boolean
  showThumbnails: boolean
}>()

const emit = defineEmits<{
  (e: 'open'): void
  (e: 'prev'): void
  (e: 'next'): void
  (e: 'zoomIn'): void
  (e: 'zoomOut'): void
  (e: 'fit'): void
  (e: 'actualSize'): void
  (e: 'rotateCw'): void
  (e: 'rotateCcw'): void
  (e: 'delete'): void
  (e: 'copy'): void
  (e: 'saveAs'): void
  (e: 'rename'): void
  (e: 'toggleInfo'): void
  (e: 'toggleThumbnails'): void
  (e: 'toggleFullscreen'): void
  (e: 'about'): void
}>()
</script>

<template>
  <header class="toolbar">
    <button title="打开 (Ctrl+O)" @click="emit('open')">📂 打开</button>
    <span class="sep" />
    <button title="上一张 (←)" :disabled="!hasImage" @click="emit('prev')">◀</button>
    <button title="下一张 (→)" :disabled="!hasImage" @click="emit('next')">▶</button>
    <span class="sep" />
    <button title="缩小 (-)" :disabled="!hasImage" @click="emit('zoomOut')">－</button>
    <button title="放大 (+)" :disabled="!hasImage" @click="emit('zoomIn')">＋</button>
    <button title="适应窗口 (F)" :disabled="!hasImage" @click="emit('fit')">⤢</button>
    <button title="100% (1)" :disabled="!hasImage" @click="emit('actualSize')">1:1</button>
    <span class="sep" />
    <button title="逆时针旋转 (Shift+R)" :disabled="!hasImage" @click="emit('rotateCcw')">⟲</button>
    <button title="顺时针旋转 (R)" :disabled="!hasImage" @click="emit('rotateCw')">⟳</button>
    <span class="sep" />
    <button title="复制路径 (Ctrl+C)" :disabled="!hasImage" @click="emit('copy')">📋</button>
    <button title="另存为 (Ctrl+S)" :disabled="!hasImage" @click="emit('saveAs')">💾</button>
    <button title="重命名 (F2)" :disabled="!hasImage" @click="emit('rename')">✎</button>
    <button
      class="danger"
      title="删除到回收站 (Delete)"
      :disabled="!hasImage"
      @click="emit('delete')"
    >🗑</button>
    <span class="spacer" />
    <button
      title="切换缩略图条 (T)"
      :class="{ active: showThumbnails }"
      @click="emit('toggleThumbnails')"
    >▤</button>
    <button
      title="切换信息面板 (I)"
      :class="{ active: showInfo }"
      @click="emit('toggleInfo')"
    >ⓘ</button>
    <button title="全屏 (F11)" @click="emit('toggleFullscreen')">⛶</button>
    <button title="关于" @click="emit('about')">?</button>
  </header>
</template>

<style scoped>
.toolbar {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 6px 8px;
  background: #1a1a20;
  border-bottom: 1px solid #2a2a30;
  color: #d0d0d8;
  user-select: none;
  flex-shrink: 0;
}
button {
  min-width: 32px;
  height: 30px;
  padding: 0 8px;
  background: transparent;
  color: #d0d0d8;
  border: 1px solid transparent;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: background 100ms, border-color 100ms;
}
button:hover:not(:disabled) {
  background: #26262d;
  border-color: #3a3a44;
}
button:disabled {
  color: #4a4a55;
  cursor: not-allowed;
}
button.active {
  background: #2a3a55;
  border-color: #3a5580;
}
button.danger:hover:not(:disabled) {
  background: #4a1e1e;
  border-color: #7a2828;
}
.sep {
  width: 1px;
  height: 20px;
  background: #2a2a30;
  margin: 0 4px;
}
.spacer {
  flex: 1;
}
</style>
