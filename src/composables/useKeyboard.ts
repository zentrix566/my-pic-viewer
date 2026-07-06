// 全局快捷键：在 App.vue 挂载时注册一次
import { onBeforeUnmount, onMounted } from 'vue'

export interface KeyHandlers {
  onPrev: () => void
  onNext: () => void
  onFirst: () => void
  onLast: () => void
  onJump: (offset: number) => void
  onZoomIn: () => void
  onZoomOut: () => void
  onActualSize: () => void
  onFit: () => void
  onRotateCW: () => void
  onRotateCCW: () => void
  onDelete: () => void
  onOpen: () => void
  onCopy: () => void
  onSaveAs: () => void
  onRename: () => void
  onToggleInfo: () => void
  onToggleThumbnails: () => void
  onToggleFullscreen: () => void
  onExitFullscreen: () => void
}

export function useKeyboard(h: KeyHandlers) {
  function isEditingElement(el: EventTarget | null): boolean {
    if (!(el instanceof HTMLElement)) return false
    const tag = el.tagName
    return (
      tag === 'INPUT' || tag === 'TEXTAREA' || el.isContentEditable
    )
  }

  function handler(e: KeyboardEvent) {
    if (isEditingElement(e.target)) return
    const k = e.key
    const ctrl = e.ctrlKey || e.metaKey
    const shift = e.shiftKey

    // Ctrl 组合键优先
    if (ctrl) {
      switch (k.toLowerCase()) {
        case 'o':
          e.preventDefault(); h.onOpen(); return
        case 'c':
          e.preventDefault(); h.onCopy(); return
        case 's':
          e.preventDefault(); h.onSaveAs(); return
      }
    }

    switch (k) {
      case 'ArrowLeft':
      case 'a':
      case 'A':
        e.preventDefault(); h.onPrev(); break
      case 'ArrowRight':
      case 'd':
      case 'D':
        e.preventDefault(); h.onNext(); break
      case 'Home':
        e.preventDefault(); h.onFirst(); break
      case 'End':
        e.preventDefault(); h.onLast(); break
      case 'PageUp':
        e.preventDefault(); h.onJump(-10); break
      case 'PageDown':
        e.preventDefault(); h.onJump(10); break
      case '+':
      case '=':
        e.preventDefault(); h.onZoomIn(); break
      case '-':
      case '_':
        e.preventDefault(); h.onZoomOut(); break
      case '0':
      case '1':
        e.preventDefault(); h.onActualSize(); break
      case 'f':
      case 'F':
        e.preventDefault(); h.onFit(); break
      case 'r':
      case 'R':
        e.preventDefault()
        if (shift) h.onRotateCCW()
        else h.onRotateCW()
        break
      case 'Delete':
        e.preventDefault(); h.onDelete(); break
      case 'F2':
        e.preventDefault(); h.onRename(); break
      case 'i':
      case 'I':
        e.preventDefault(); h.onToggleInfo(); break
      case 't':
      case 'T':
        e.preventDefault(); h.onToggleThumbnails(); break
      case 'F11':
        e.preventDefault(); h.onToggleFullscreen(); break
      case 'Escape':
        h.onExitFullscreen(); break
    }
  }

  onMounted(() => window.addEventListener('keydown', handler))
  onBeforeUnmount(() => window.removeEventListener('keydown', handler))
}
