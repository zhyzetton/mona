// 全局消息通知:模块级 pub/sub store,不依赖 React 上下文,
// 任何模块直接 import { toast } 调用即可,由 <Toaster /> 负责渲染
export type ToastKind = "success" | "error";

export interface ToastItem {
  id: number;
  kind: ToastKind;
  message: string;
  // 退出动画标记:置 true 后短暂延迟再移除,让淡出可见
  leaving?: boolean;
}

const SUCCESS_AUTO_DISMISS_MS = 3200;
const LEAVE_ANIMATION_MS = 220;
const MAX_TOASTS = 5;

let toasts: ToastItem[] = [];
let nextId = 1;
const listeners = new Set<() => void>();

function setToasts(next: ToastItem[]) {
  toasts = next;
  listeners.forEach((l) => l());
}

export function subscribeToasts(listener: () => void): () => void {
  listeners.add(listener);
  return () => {
    listeners.delete(listener);
  };
}

// useSyncExternalStore 的快照:每次变更整体替换数组,保证引用变化触发渲染
export function getToastSnapshot(): ToastItem[] {
  return toasts;
}

export function showToast(kind: ToastKind, message: string) {
  const id = nextId++;
  // 超出上限时直接丢弃最旧的一条,防止错误消息堆积成墙
  setToasts([...toasts.slice(Math.max(0, toasts.length - MAX_TOASTS + 1)), { id, kind, message }]);
  if (kind === "success") {
    window.setTimeout(() => dismissToast(id), SUCCESS_AUTO_DISMISS_MS);
  }
}

export function dismissToast(id: number) {
  const target = toasts.find((t) => t.id === id);
  if (!target || target.leaving) return;
  setToasts(toasts.map((t) => (t.id === id ? { ...t, leaving: true } : t)));
  window.setTimeout(() => {
    setToasts(toasts.filter((t) => t.id !== id));
  }, LEAVE_ANIMATION_MS);
}

export const toast = {
  success: (message: string) => showToast("success", message),
  error: (message: string) => showToast("error", message),
};
