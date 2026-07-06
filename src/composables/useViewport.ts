// 视口变换状态（缩放/平移/旋转）与相关算法
// 采用 CSS transform: translate(x,y) scale(s) rotate(r)，硬件加速
import { computed, ref } from 'vue'

const MIN_SCALE = 0.1
const MAX_SCALE = 32
const ZOOM_STEP = 1.15

const scale = ref(1)
const translateX = ref(0)
const translateY = ref(0)
const rotation = ref(0)

// 图像自然尺寸与容器尺寸（外部组件在 load / resize 时写入）
const naturalWidth = ref(0)
const naturalHeight = ref(0)
const containerWidth = ref(0)
const containerHeight = ref(0)

export function useViewport() {
  const transformStyle = computed(() => ({
    transform: `translate(${translateX.value}px, ${translateY.value}px) scale(${scale.value}) rotate(${rotation.value}deg)`
  }))

  const isZoomedIn = computed(() => scale.value > fitScale())

  /// 计算"适应窗口"的缩放（考虑旋转后的图像方向）
  function fitScale() {
    if (!naturalWidth.value || !naturalHeight.value) return 1
    const rotated = Math.abs(rotation.value) % 180 === 90
    const w = rotated ? naturalHeight.value : naturalWidth.value
    const h = rotated ? naturalWidth.value : naturalHeight.value
    const sx = containerWidth.value / w
    const sy = containerHeight.value / h
    return Math.min(sx, sy, 1) // 小图不放大，只缩小到适合
  }

  /// 重置到"适应窗口"
  function fitToWindow() {
    scale.value = fitScale()
    translateX.value = 0
    translateY.value = 0
  }

  /// 回到 100%
  function actualSize() {
    scale.value = 1
    translateX.value = 0
    translateY.value = 0
  }

  /// 以给定"容器坐标"为中心缩放
  function zoomAt(clientX: number, clientY: number, factor: number) {
    const newScale = clamp(scale.value * factor, MIN_SCALE, MAX_SCALE)
    if (newScale === scale.value) return
    // 反算：让鼠标下方那一像素在缩放前后保持不动
    // 图像相对容器中心的偏移由 translate 表示，我们把「鼠标点」当作缩放原点
    const cx = containerWidth.value / 2
    const cy = containerHeight.value / 2
    // 鼠标相对容器中心
    const mx = clientX - cx
    const my = clientY - cy
    const ratio = newScale / scale.value
    translateX.value = mx - (mx - translateX.value) * ratio
    translateY.value = my - (my - translateY.value) * ratio
    scale.value = newScale
  }

  function zoomIn() {
    zoomAt(containerWidth.value / 2, containerHeight.value / 2, ZOOM_STEP)
  }

  function zoomOut() {
    zoomAt(containerWidth.value / 2, containerHeight.value / 2, 1 / ZOOM_STEP)
  }

  /// 平移
  function pan(dx: number, dy: number) {
    translateX.value += dx
    translateY.value += dy
  }

  /// 旋转
  function rotateBy(deg: number) {
    rotation.value = (rotation.value + deg + 360) % 360
    fitToWindow()
  }

  /// 换图时调用，重置一切
  function reset(newWidth: number, newHeight: number) {
    naturalWidth.value = newWidth
    naturalHeight.value = newHeight
    rotation.value = 0
    fitToWindow()
  }

  /// 容器尺寸变化时调用
  function setContainerSize(w: number, h: number) {
    containerWidth.value = w
    containerHeight.value = h
    if (naturalWidth.value) {
      // 保持"适应窗口"，除非用户已放大
      if (!isZoomedIn.value) fitToWindow()
    }
  }

  return {
    scale,
    translateX,
    translateY,
    rotation,
    naturalWidth,
    naturalHeight,
    containerWidth,
    containerHeight,
    transformStyle,
    fitToWindow,
    actualSize,
    zoomAt,
    zoomIn,
    zoomOut,
    pan,
    rotateBy,
    reset,
    setContainerSize
  }
}

function clamp(v: number, min: number, max: number) {
  return Math.max(min, Math.min(max, v))
}
