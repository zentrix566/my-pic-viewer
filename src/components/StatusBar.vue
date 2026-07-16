<script setup lang="ts">
import { baseName } from '../utils/path'

defineProps<{
  index: number
  total: number
  scale: number
  width: number
  height: number
  fileName: string
  goodDir: string
  badDir: string
}>()
</script>

<template>
  <footer class="status-bar">
    <span v-if="total > 0">{{ index + 1 }} / {{ total }}</span>
    <span v-else>无图片</span>
    <span class="sep">|</span>
    <span>{{ fileName || '-' }}</span>
    <span class="sep">|</span>
    <span v-if="width && height">{{ width }} × {{ height }}</span>
    <span class="spacer" />
    <span v-if="goodDir" class="dir-badge good" title="合适目录">✅ {{ baseName(goodDir) }}</span>
    <span v-if="badDir" class="dir-badge bad" title="不合适目录">❌ {{ baseName(badDir) }}</span>
    <span class="sep" v-if="goodDir || badDir" />
    <span>{{ Math.round(scale * 100) }}%</span>
  </footer>
</template>

<style scoped>
.status-bar {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 12px;
  background: #17171c;
  border-top: 1px solid #2a2a30;
  color: #9a9aa4;
  font-size: 12px;
  height: 26px;
  flex-shrink: 0;
  user-select: none;
}
.sep {
  color: #4a4a55;
}
.spacer {
  flex: 1;
}
.dir-badge {
  font-size: 11px;
  padding: 1px 6px;
  border-radius: 3px;
  max-width: 160px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.dir-badge.good {
  background: rgba(60, 160, 80, 0.18);
  color: #6cdb8a;
}
.dir-badge.bad {
  background: rgba(180, 60, 60, 0.18);
  color: #db6c6c;
}
</style>
