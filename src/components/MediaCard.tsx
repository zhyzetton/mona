import { Film } from "lucide-react";
import { assetURL } from "../api";
import type { Media } from "../types";

// 后端返回的 media_type 是枚举字符串,映射成中文标签
const TYPE_LABELS: Record<string, string> = {
  Movie: "电影",
  Series: "剧集",
  Anime: "动画",
  Variety: "综艺",
  Documentary: "纪录片",
  Personal: "个人",
};

export function mediaTypeLabel(type: string | null | undefined): string {
  if (!type) return "未知";
  return TYPE_LABELS[type] ?? type;
}

// 响应式海报墙:海报基准宽 160px,列数随容器宽度自动增减,而不是整墙等比放大
export const POSTER_GRID =
  "grid grid-cols-[repeat(auto-fill,minmax(160px,1fr))] gap-x-4 gap-y-7";

export function PosterCard({
  media,
  onClick,
  index = 0,
}: {
  media: Media;
  onClick?: () => void;
  // 用于网格入场时的错峰延迟;只取前一小段,避免列表尾部延迟过长
  index?: number;
}) {
  return (
    <div
      className="group cursor-pointer animate-fade-up"
      style={{ animationDelay: `${Math.min(index, 11) * 40}ms` }}
      onClick={onClick}
    >
      <div className="relative aspect-[2/3] rounded-xl overflow-hidden bg-tint/5 ring-1 ring-tint/10 transition-all duration-300 group-hover:ring-tint/25 group-hover:shadow-2xl group-hover:shadow-black/40 group-hover:-translate-y-1">
        {media.poster_path ? (
          <img
            src={assetURL(media.poster_path)}
            alt={media.title}
            className="w-full h-full object-cover transition-transform duration-500 ease-out group-hover:scale-[1.06]"
            loading="lazy"
          />
        ) : (
          <div className="w-full h-full flex items-center justify-center bg-tint/5 text-mute">
            <Film size={32} />
          </div>
        )}
      </div>

      <p className="mt-2.5 text-sm font-medium text-body group-hover:text-strong truncate transition-colors">
        {media.title}
      </p>
      <p className="text-xs text-secondary truncate">
        {[media.year, media.duration].filter(Boolean).join(" · ")}
      </p>
    </div>
  );
}
