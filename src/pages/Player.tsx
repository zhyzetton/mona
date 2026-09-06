import { useState } from "react";
import { ChevronLeft, ExternalLink } from "lucide-react";
import { api, assetURL } from "../api";
import { appWindow, WindowControls } from "../components/TitleBar";
import type { Media } from "../types";

export default function Player({
  media,
  onBack,
}: {
  media: Media;
  onBack: () => void;
}) {
  const [error, setError] = useState<string | null>(null);

  const openExternal = async () => {
    setError(null);
    try {
      await api.playVideo(media.id!);
    } catch (e) {
      console.error(e);
      setError(String(e));
    }
  };

  return (
    <div className="fixed inset-0 z-[60] bg-black flex flex-col animate-fade-in">
      {/* 顶部栏:悬浮在视频上,渐变遮罩保证可读;整条可拖拽移动窗口 */}
      <div
        data-tauri-drag-region
        onDoubleClick={() => appWindow.toggleMaximize().catch(() => {})}
        className="absolute top-0 inset-x-0 z-10 h-12 flex items-center gap-2 bg-gradient-to-b from-black/80 to-transparent select-none"
      >
        <button
          onClick={onBack}
          className="ml-2 p-2.5 rounded-full bg-white/5 backdrop-blur-md ring-1 ring-white/10 text-white/80 hover:text-white hover:bg-white/15 hover:scale-105 active:scale-95 cursor-pointer transition-all"
          aria-label="返回"
        >
          <ChevronLeft size={20} />
        </button>
        <span
          data-tauri-drag-region
          className="flex-1 min-w-0 text-white/80 text-sm font-medium truncate px-2"
        >
          {media.title}
        </span>
        <div className="h-full flex items-stretch">
          <WindowControls overlay />
        </div>
      </div>

      {/* 视频主体 */}
      <div className="flex-1 flex items-center justify-center min-h-0">
        <video
          key={media.file_path}
          src={assetURL(media.file_path)}
          controls
          autoPlay
          className="w-full h-full max-h-full object-contain bg-black"
          onError={() =>
            setError("此视频无法在应用内播放(格式或编码不支持),可改用系统播放器。")
          }
        />
      </div>

      {error && (
        <div className="absolute bottom-6 inset-x-0 flex justify-center px-6">
          {/* 播放页恒为黑色背景,错误条固定深色玻璃,不随主题 */}
          <div className="rounded-xl px-5 py-3 flex items-center gap-4 animate-fade-up bg-black/60 backdrop-blur-md ring-1 ring-white/15">
            <span className="text-red-400 text-sm">{error}</span>
            <button
              onClick={openExternal}
              className="flex items-center gap-1.5 px-4 py-2 rounded-lg bg-white/10 hover:bg-white/20 text-white text-sm transition-colors"
            >
              <ExternalLink size={14} /> 用系统播放器打开
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
