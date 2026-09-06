import { useEffect, useState } from "react";
import { Copy, Minus, Square, X } from "lucide-react";
import { getCurrentWindow } from "@tauri-apps/api/window";

export const appWindow = getCurrentWindow();

export function useMaximized() {
  const [maximized, setMaximized] = useState(false);
  useEffect(() => {
    let disposed = false;
    appWindow.isMaximized().then((v) => !disposed && setMaximized(v)).catch(() => {});
    const un = appWindow.onResized(() => {
      appWindow.isMaximized().then((v) => !disposed && setMaximized(v)).catch(() => {});
    });
    return () => {
      disposed = true;
      un.then((fn) => fn()).catch(() => {});
    };
  }, []);
  return maximized;
}

/** 最小化/最大化/关闭三个窗口按钮;overlay 用于悬浮在深色内容(如播放页)上的场景 */
export function WindowControls({ overlay = false }: { overlay?: boolean }) {
  const maximized = useMaximized();
  const btn = overlay
    ? "w-12 flex items-center justify-center text-white/80 hover:text-white hover:bg-white/15 transition-colors"
    : "w-12 flex items-center justify-center text-secondary hover:text-strong hover:bg-tint/10 transition-colors";
  return (
    <div className="flex h-full items-stretch">
      <button onClick={() => appWindow.minimize().catch(() => {})} aria-label="最小化" className={btn}>
        <Minus size={15} />
      </button>
      <button
        onClick={() => appWindow.toggleMaximize().catch(() => {})}
        aria-label={maximized ? "还原" : "最大化"}
        className={btn}
      >
        {maximized ? <Copy size={13} /> : <Square size={13} />}
      </button>
      <button
        onClick={() => appWindow.close().catch(() => {})}
        aria-label="关闭"
        className={`${btn} hover:bg-red-500`}
      >
        <X size={16} />
      </button>
    </div>
  );
}

/** 应用主标题栏:左侧留给拖拽,右侧窗口控制(双击空白处最大化/还原) */
export function TitleBar() {
  return (
    <header
      data-tauri-drag-region
      onDoubleClick={() => appWindow.toggleMaximize().catch(() => {})}
      className="h-10 shrink-0 flex items-stretch justify-end bg-base border-b border-tint/10 select-none"
    >
      <WindowControls />
    </header>
  );
}
