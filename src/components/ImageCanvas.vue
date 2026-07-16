<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { convertFileSrc } from '@tauri-apps/api/core'
import { useViewport } from '../composables/useViewport'

const props = defineProps<{
  path: string
}>()

const emit = defineEmits<{
  (e: 'dblclick'): void
  (e: 'loaded', payload: { width: number; height: number }): void
  (e: 'copy'): void
}>()

const viewport = useViewport()
const containerRef = ref<HTMLDivElement | null>(null)
const imgRef = ref<HTMLImageElement | null>(null)

const src = computed(() => (props.path ? convertFileSrc(props.path) : ''))

// ------- 右键菜单 -------
const contextMenu = ref({ show: false, x: 0, y: 0 })

function onContextMenu(e: MouseEvent) {
  e.preventDefault()
  if (!props.path) return
  contextMenu.value = { show: true, x: e.clientX, y: e.clientY }
}

function closeContextMenu() {
  contextMenu.value.show = false
}

function onCopy() {
  closeContextMenu()
  emit('copy')
}

// 点击页面其他地方关闭菜单
function onGlobalClick() {
  closeContextMenu()
}
onMounted(() => window.addEventListener('click', onGlobalClick))
onBeforeUnmount(() => window.removeEventListener('click', onGlobalClick))

// ------- 容器尺寸监听 -------
let resizeObserver: ResizeObserver | null = null
onMounted(() => {
  if (containerRef.value) {
    resizeObserver = new ResizeObserver(() => {
      if (!containerRef.value) return
      const rect = containerRef.value.getBoundingClientRect()
      viewport.setContainerSize(rect.width, rect.height)
    })
    resizeObserver.observe(containerRef.value)
    const rect = containerRef.value.getBoundingClientRect()
    viewport.setContainerSize(rect.width, rect.height)
  }
})
onBeforeUnmount(() => resizeObserver?.disconnect())

// ------- 图片加载完成，重置视口 -------
function onLoad() {
  const el = imgRef.value
  if (!el) return
  viewport.reset(el.naturalWidth, el.naturalHeight)
  emit('loaded', { width: el.naturalWidth, height: el.naturalHeight })
}

// 切换图片时若命中缓存不会触发 load，需要主动 reset
watch(() => props.path, () => {
  const el = imgRef.value
  if (el && el.complete && el.naturalWidth > 0) {
    // 等下一帧再读取，保证 naturalWidth 是新图片的
    requestAnimationFrame(() => onLoad())
  }
})

// ------- 滚轮以鼠标位置为中心缩放 -------
function onWheel(e: WheelEvent) {
  e.preventDefault()
  if (!containerRef.value) return
  const rect = containerRef.value.getBoundingClientRect()
  const x = e.clientX - rect.left
  const y = e.clientY - rect.top
  const factor = e.deltaY < 0 ? 1.15 : 1 / 1.15
  viewport.zoomAt(x, y, factor)
}

// ------- 拖拽平移 -------
const dragging = ref(false)
const dragStart = ref<{ x: number; y: number; tx: number; ty: number } | null>(null)

function onPointerDown(e: PointerEvent) {
  if (e.button !== 0) return
  dragging.value = true
  dragStart.value = {
    x: e.clientX,
    y: e.clientY,
    tx: viewport.translateX.value,
    ty: viewport.translateY.value
  }
  ;(e.currentTarget as HTMLElement).setPointerCapture(e.pointerId)
}

function onPointerMove(e: PointerEvent) {
  if (!dragging.value || !dragStart.value) return
  const dx = e.clientX - dragStart.value.x
  const dy = e.clientY - dragStart.value.y
  viewport.translateX.value = dragStart.value.tx + dx
  viewport.translateY.value = dragStart.value.ty + dy
}

function onPointerUp(e: PointerEvent) {
  dragging.value = false
  dragStart.value = null
  try {
    ;(e.currentTarget as HTMLElement).releasePointerCapture(e.pointerId)
  } catch {
    /* 有时 pointerId 已经释放 */
  }
}

function onDblClick() {
  emit('dblclick')
}
</script>

<template>
  <div
    ref="containerRef"
    class="image-canvas"
    :class="{ dragging }"
    @wheel="onWheel"
    @pointerdown="onPointerDown"
    @pointermove="onPointerMove"
    @pointerup="onPointerUp"
    @pointercancel="onPointerUp"
    @dblclick="onDblClick"
    @contextmenu="onContextMenu"
  >
    <img
      v-if="src"
      ref="imgRef"
      :src="src"
      :style="viewport.transformStyle.value"
      class="image"
      draggable="false"
      @load="onLoad"
    />
    <div v-else class="empty-hint">
      <p>拖拽图片到此处，或按 Ctrl+O 打开</p>
    </div>

    <!-- 右键菜单 -->
    <Teleport to="body">
      <div
        v-if="contextMenu.show"
        class="ctx-menu"
        :style="{ left: contextMenu.x + 'px', top: contextMenu.y + 'px' }"
      >
        <button class="ctx-item" @click="onCopy">📋 复制图片</button>
      </div>
    </Teleport>
  </div>
</template>

<style scoped>
.image-canvas {
  position: relative;
  width: 100%;
  height: 100%;
  overflow: hidden;
  background: #2a2a30;
  cursor: grab;
  user-select: none;
  display: flex;
  align-items: center;
  justify-content: center;
}
.image-canvas.dragging {
  cursor: grabbing;
}
.image {
  transform-origin: center center;
  max-width: none;
  max-height: none;
  pointer-events: none;
  image-rendering: auto;
  will-change: transform;
  flex-shrink: 0;
}
.empty-hint {
  color: #6a6a75;
  font-size: 14px;
  pointer-events: none;
}

/* ---------- 自定义右键菜单 ---------- */
.ctx-menu {
  position: fixed;
  z-index: 9999;
  min-width: 140px;
  background: #1e1e26;
  border: 1px solid #3a3a44;
  border-radius: 6px;
  padding: 4px 0;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.5);
  overflow: hidden;
}
.ctx-item {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  padding: 8px 14px;
  background: transparent;
  color: #d0d0d8;
  border: none;
  cursor: pointer;
  font-size: 13px;
  white-space: nowrap;
  text-align: left;
}
.ctx-item:hover {
  background: #2a3a55;
}
</style>
