import { useSyncExternalStore } from "react";
import { CheckCircle2, XCircle, X } from "lucide-react";
import {
  dismissToast,
  getToastSnapshot,
  subscribeToasts,
  type ToastItem,
} from "../toast";

// 全局消息容器:挂在 App 根部,层级高于播放页,任何页面触发的消息都浮在最上层。
// 容器本身不拦截点击(指针穿透),只有消息卡片可交互
export function Toaster() {
  const toasts = useSyncExternalStore(subscribeToasts, getToastSnapshot);

  return (
    <div className="fixed top-14 right-5 z-[80] flex flex-col items-end gap-2.5 pointer-events-none">
      {toasts.map((t) => (
        <ToastCard key={t.id} item={t} />
      ))}
    </div>
  );
}

function ToastCard({ item }: { item: ToastItem }) {
  const ok = item.kind === "success";
  return (
    <div
      role="status"
      className={`pointer-events-auto flex items-center gap-2.5 pl-3.5 pr-2 py-2.5 rounded-xl shadow-2xl shadow-black/30 glass animate-toast-in ${
        item.leaving ? "toast-leaving" : ""
      }`}
    >
      {ok ? (
        <CheckCircle2 size={17} className="text-ok shrink-0" />
      ) : (
        <XCircle size={17} className="text-err shrink-0" />
      )}
      <span className="text-sm text-body max-w-80 break-all">{item.message}</span>
      <button
        onClick={() => dismissToast(item.id)}
        aria-label="关闭"
        className={`ml-0.5 w-6 h-6 shrink-0 flex items-center justify-center rounded-lg text-mute hover:text-strong hover:bg-tint/10 transition-colors ${
          ok ? "opacity-50" : ""
        }`}
      >
        <X size={14} />
      </button>
    </div>
  );
}
