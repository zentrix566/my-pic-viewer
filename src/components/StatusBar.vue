<script setup lang="ts">
defineProps<{
  index: number
  total: number
  scale: number
  width: number
  height: number
  fileName: string
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
</style>
