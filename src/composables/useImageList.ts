// 图片列表状态管理：当前路径、列表、索引、切换、删除同步
import { computed, ref } from 'vue'
import { tauriApi } from './useTauriApi'

const items = ref<string[]>([])
const currentIndex = ref(0)

/// 支持从任意组件复用的图片列表 store
export function useImageList() {
  const currentPath = computed(() =>
    items.value.length > 0 ? items.value[currentIndex.value] : ''
  )
  const total = computed(() => items.value.length)
  const hasImage = computed(() => items.value.length > 0)

  /// 打开一张图片（或一个目录），扫描其所在目录并定位到当前项
  async function openPath(path: string) {
    const res = await tauriApi.listImagesInDir(path)
    items.value = res.items
    currentIndex.value = res.currentIndex
  }

  /// 上一张（循环）
  function prev() {
    if (items.value.length === 0) return
    currentIndex.value =
      (currentIndex.value - 1 + items.value.length) % items.value.length
  }

  /// 下一张（循环）
  function next() {
    if (items.value.length === 0) return
    currentIndex.value = (currentIndex.value + 1) % items.value.length
  }

  function first() {
    if (items.value.length > 0) currentIndex.value = 0
  }

  function last() {
    if (items.value.length > 0) currentIndex.value = items.value.length - 1
  }

  function jump(offset: number) {
    if (items.value.length === 0) return
    const n = items.value.length
    currentIndex.value = ((currentIndex.value + offset) % n + n) % n
  }

  function goTo(index: number) {
    if (index >= 0 && index < items.value.length) currentIndex.value = index
  }

  /// 删除当前项，前端同步维护列表；末位删除后回退到前一张
  function removeCurrent() {
    if (items.value.length === 0) return
    items.value.splice(currentIndex.value, 1)
    if (currentIndex.value >= items.value.length) {
      currentIndex.value = Math.max(0, items.value.length - 1)
    }
  }

  /// 重命名当前项：把 items 里对应路径换成新的
  function updateCurrentPath(newPath: string) {
    if (items.value.length === 0) return
    items.value.splice(currentIndex.value, 1, newPath)
  }

  return {
    items,
    currentIndex,
    currentPath,
    total,
    hasImage,
    openPath,
    prev,
    next,
    first,
    last,
    jump,
    goTo,
    removeCurrent,
    updateCurrentPath
  }
}
