<script setup lang="ts">
import { computed } from 'vue'
import { convertFileSrc } from '@tauri-apps/api/core'

const props = defineProps<{
  items: string[]
  currentIndex: number
}>()

const emit = defineEmits<{
  (e: 'select', index: number): void
}>()

/// 从完整路径取文件名
function baseName(p: string): string {
  const s = p.replace(/\\/g, '/')
  const i = s.lastIndexOf('/')
  return i >= 0 ? s.slice(i + 1) : s
}

const previewItems = computed(() =>
  props.items.map((p, i) => ({
    src: convertFileSrc(p),
    name: baseName(p),
    index: i
  }))
)
</script>

<template>
  <div class="thumbnail-strip">
    <div
      v-for="item in previewItems"
      :key="item.index"
      class="thumb"
      :class="{ active: item.index === currentIndex }"
      :title="item.name"
      @click="emit('select', item.index)"
    >
      <img :src="item.src" loading="lazy" draggable="false" />
    </div>
  </div>
</template>

<style scoped>
.thumbnail-strip {
  display: flex;
  gap: 4px;
  padding: 6px 8px;
  overflow-x: auto;
  overflow-y: hidden;
  background: #1a1a20;
  border-top: 1px solid #2a2a30;
  scrollbar-width: thin;
}
.thumb {
  flex: 0 0 auto;
  width: 96px;
  height: 72px;
  border: 2px solid transparent;
  border-radius: 4px;
  overflow: hidden;
  cursor: pointer;
  background: #0d0d10;
  transition: border-color 120ms;
}
.thumb:hover {
  border-color: #444450;
}
.thumb.active {
  border-color: #5aa9ff;
}
.thumb img {
  width: 100%;
  height: 100%;
  object-fit: contain;
  pointer-events: none;
}
</style>
