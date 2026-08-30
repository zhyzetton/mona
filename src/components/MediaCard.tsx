import { Film } from "lucide-react";
import { assetURL } from "../api";
import type { Media } from "../types";

// 后端返回的 media_type 是枚举字符串,映射成中文标签
const TYPE_LABELS: Record<string, string> = {
  Movie: "电影",
  Series: "剧集",
  Anime: "动画",
  Local: "本地",
};

export function mediaTypeLabel(type: string): string {
  return TYPE_LABELS[type] ?? type ?? "未知";
}

export function PosterCard({
  media,
  onClick,
}: {
  media: Media;
  onClick?: () => void;
}) {
  return (
    <div className="group cursor-pointer" onClick={onClick}>
      <div className="aspect-[2/3] rounded-xl overflow-hidden bg-slate-100 relative shadow-sm group-hover:shadow-md group-hover:-translate-y-0.5 transition-all">
        {media.poster_path ? (
          <img
            src={assetURL(media.poster_path)}
            alt={media.title}
            className="w-full h-full object-cover"
            loading="lazy"
          />
        ) : (
          <div className="w-full h-full flex items-center justify-center bg-gradient-to-br from-slate-100 to-slate-200 text-slate-300">
            <Film size={32} />
          </div>
        )}
        <span className="absolute top-2 right-2 px-2 py-0.5 rounded-md bg-black/55 text-white text-[10px]">
          {mediaTypeLabel(media.media_type)}
        </span>
      </div>
      <p className="mt-2 text-sm font-medium text-slate-800 truncate">{media.title}</p>
      <p className="text-xs text-slate-400 truncate">
        {[media.year, media.duration].filter(Boolean).join(" · ")}
      </p>
    </div>
  );
}
