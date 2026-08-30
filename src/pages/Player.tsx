import { useState } from "react";
import { ChevronLeft } from "lucide-react";
import { api, assetURL } from "../api";
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
    <div className="fixed inset-0 z-[60] bg-black flex flex-col">
      {/* 顶部栏 */}
      <div className="flex items-center justify-between px-6 py-4 shrink-0">
        <button
          onClick={onBack}
          className="p-2.5 rounded-full text-white/80 hover:text-white hover:bg-white/15 cursor-pointer transition-colors"
          aria-label="返回"
        >
          <ChevronLeft size={28} />
        </button>
        <span className="text-white/70 text-sm truncate max-w-[60%]">
          {media.title}
        </span>
      </div>

      {/* 视频主体 */}
      <div className="flex-1 flex items-center justify-center px-6 pb-6 min-h-0">
        <video
          key={media.file_path}
          src={assetURL(media.file_path)}
          controls
          autoPlay
          className="w-full h-full max-h-full object-contain rounded-lg bg-black"
          onError={() =>
            setError("此视频无法在应用内播放(格式或编码不支持),可改用系统播放器。")
          }
        />
      </div>

      {error && (
        <div className="shrink-0 px-6 pb-5 flex items-center justify-center gap-4">
          <span className="text-red-400 text-sm">{error}</span>
          <button
            onClick={openExternal}
            className="px-4 py-2 rounded-lg bg-white/10 hover:bg-white/20 text-white text-sm transition-colors"
          >
            用系统播放器打开
          </button>
        </div>
      )}
    </div>
  );
}
