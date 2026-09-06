import { useEffect, useState } from "react";
import { ChevronRight, Film } from "lucide-react";
import { api, assetURL } from "../api";
import { POSTER_GRID, PosterCard, mediaTypeLabel } from "../components/MediaCard";
import type { Media } from "../types";

export default function Home({
  search,
  onOpen,
  refreshKey,
  onShowAll,
}: {
  search: string;
  onOpen: (media: Media) => void;
  refreshKey: number;
  onShowAll?: () => void;
}) {
  const [media, setMedia] = useState<Media[]>([]);
  const [recentPlayed, setRecentPlayed] = useState<Media[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    api
      .getVideos()
      .then(setMedia)
      .catch((e) => {
        console.error(e);
        setError(String(e));
      });
    // 播放历史加载失败不影响主列表,只静默降级为不显示"正在观看"
    api
      .getRecentPlayed()
      .then(setRecentPlayed)
      .catch((e) => console.error(e));
  }, [refreshKey]);

  const kw = search.toLowerCase();
  const filtered = media.filter(
    (m) =>
      !kw ||
      m.title.toLowerCase().includes(kw) ||
      m.file_path.toLowerCase().includes(kw),
  );

  // 正在观看:后端真实播放记录,取最近三个有海报的;没播过就不显示
  const watch = recentPlayed.filter((m) => m.poster_path).slice(0, 3);
  // 最近添加:按入库时间倒序,同秒入库的按 id 兜底
  const recent = [...filtered]
    .sort(
      (a, b) =>
        b.added_at - a.added_at || (b.id ?? 0) - (a.id ?? 0),
    )
    .slice(0, 10);

  return (
    <div className="px-8 lg:px-10 py-8">
      <header className="mb-8 animate-fade-in">
        <h1 className="text-3xl font-bold text-strong tracking-tight">主屏幕</h1>
      </header>

      {error && (
        <div className="text-err text-sm mb-6 bg-err/10 border border-err/20 rounded-xl px-4 py-3">
          加载失败: {error}
        </div>
      )}

      {/* 正在观看: 大图横幅(图片之上保持黑色覆盖层,两种主题都可读) */}
      {watch.length > 0 && (
        <section className="mb-12">
          <h2 className="text-lg font-semibold text-strong mb-4">正在观看</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-5">
            {watch.map((m, i) => (
              <div
                key={m.id}
                className="group relative aspect-video rounded-2xl overflow-hidden cursor-pointer ring-1 ring-white/10 hover:ring-white/25 shadow-xl shadow-black/40 transition-all duration-300 animate-fade-up"
                style={{ animationDelay: `${i * 80}ms` }}
                onClick={() => onOpen(m)}
              >
                <img
                  src={assetURL(m.poster_path)}
                  alt={m.title}
                  className="absolute inset-0 w-full h-full object-cover transition-transform duration-700 ease-out group-hover:scale-[1.04]"
                />
                {/* 底部渐变,让文字可读 */}
                <div className="absolute inset-0 bg-gradient-to-t from-black/85 via-black/25 to-transparent" />

                <div className="absolute inset-x-0 bottom-0 p-5">
                  <p className="text-white font-semibold text-base truncate drop-shadow">
                    {m.title}
                  </p>
                  <p className="text-white/70 text-xs mt-1">
                    {[
                      mediaTypeLabel(m.media_type),
                      m.year,
                      m.duration,
                      m.resolution > 0 ? `${m.resolution}p` : null,
                    ]
                      .filter(Boolean)
                      .join(" · ")}
                  </p>
                </div>
              </div>
            ))}
          </div>
        </section>
      )}

      {/* 最近添加 */}
      {media.length === 0 && !error ? (
        <div className="flex flex-col items-center justify-center py-24 text-center animate-fade-in">
          <div className="w-20 h-20 rounded-2xl bg-tint/5 ring-1 ring-tint/10 flex items-center justify-center mb-6">
            <Film size={36} className="text-mute" />
          </div>
          <h2 className="text-xl font-semibold text-body mb-2">还没有媒体</h2>
          <p className="text-secondary max-w-sm text-sm leading-relaxed">
            去「设置」添加一个本地视频目录,然后到「个人」分类点「扫描」。
          </p>
        </div>
      ) : (
        <section>
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-lg font-semibold text-strong">最近添加</h2>
            {onShowAll && (
              <button
                onClick={onShowAll}
                className="flex items-center gap-0.5 text-sm text-secondary hover:text-orange-500 transition-colors"
              >
                查看全部
                <ChevronRight size={16} />
              </button>
            )}
          </div>
          <div className={POSTER_GRID}>
            {recent.map((m, i) => (
              <PosterCard key={m.id} media={m} index={i} onClick={() => onOpen(m)} />
            ))}
          </div>
        </section>
      )}
    </div>
  );
}
