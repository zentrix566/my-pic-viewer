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
}>()

const viewport = useViewport()
const containerRef = ref<HTMLDivElement | null>(null)
const imgRef = ref<HTMLImageElement | null>(null)

const src = computed(() => (props.path ? convertFileSrc(props.path) : ''))

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
  </div>
</template>

<style scoped>
.image-canvas {
  position: relative;
  width: 100%;
  height: 100%;
  overflow: hidden;
  background: #101014;
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
</style>
