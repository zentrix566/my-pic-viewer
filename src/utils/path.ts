// 路径处理小工具：从完整路径里取文件名
// 兼容 Windows 反斜杠与 Unix 斜杠；末尾带分隔符或为空时回退到原串

export function baseName(p: string): string {
  const segs = p.replace(/\\/g, '/').split('/').filter(Boolean)
  return segs.length > 0 ? segs[segs.length - 1] : p
}
