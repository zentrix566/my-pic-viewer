<script setup lang="ts">
function getDirName(p: string): string {
  const parts = p.replace(/\\/g, '/').split('/')
  return parts[parts.length - 1] || parts[parts.length - 2] || p
}

defineProps<{
  hasImage: boolean
  showInfo: boolean
  showThumbnails: boolean
  goodDir: string
  badDir: string
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
  (e: 'checkUpdate'): void
  (e: 'setGoodDir'): void
  (e: 'setBadDir'): void
  (e: 'moveToGood'): void
  (e: 'moveToBad'): void
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
    <span class="sep" />
    <button title="设置「合适」目录" @click="emit('setGoodDir')" class="cfg-good">📁✅</button>
    <button title="设置「不合适」目录" @click="emit('setBadDir')" class="cfg-bad">📁❌</button>
    <button
      v-if="goodDir"
      :title="'移动到合适目录 (Z): ' + goodDir"
      :disabled="!hasImage"
      @click="emit('moveToGood')"
      class="action-good"
    >→ ✅ {{ getDirName(goodDir) }}</button>
    <button
      v-if="badDir"
      :title="'移动到不合适目录 (X): ' + badDir"
      :disabled="!hasImage"
      @click="emit('moveToBad')"
      class="action-bad"
    >→ ❌ {{ getDirName(badDir) }}</button>
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
    <button title="检查更新" @click="emit('checkUpdate')">⏬</button>
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

/* ---------- 分类目录设置按钮（小） ---------- */
button.cfg-good,
button.cfg-bad {
  font-size: 12px;
  padding: 0 6px;
  min-width: auto;
}
button.cfg-good {
  color: #7ad89a;
}
button.cfg-bad {
  color: #d87a7a;
}

/* ---------- 移动操作按钮（醒目大按钮） ---------- */
button.action-good,
button.action-bad {
  font-size: 13px;
  padding: 0 12px;
  min-width: 56px;
  max-width: 220px;
  height: 30px;
  border-radius: 5px;
  font-weight: 600;
  letter-spacing: 0.5px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
button.action-good {
  background: #1e4a2a;
  color: #7ad89a;
  border-color: #2a6a3a;
}
button.action-good:hover:not(:disabled) {
  background: #2a6a3a;
  border-color: #4a9a5a;
}
button.action-bad {
  background: #4a1e1e;
  color: #d87a7a;
  border-color: #6a2a2a;
}
button.action-bad:hover:not(:disabled) {
  background: #6a2a2a;
  border-color: #9a4a4a;
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
