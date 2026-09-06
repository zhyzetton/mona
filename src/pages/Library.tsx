import { useEffect, useMemo, useRef, useState } from "react";
import { FolderOpen, Loader2, RefreshCw } from "lucide-react";
import { api } from "../api";
import { POSTER_GRID, PosterCard } from "../components/MediaCard";
import { toast } from "../toast";
import type { Media } from "../types";

const PAGE_SIZE = 30;

// 各分类对应的 media_type;other 表示不属于任何已知类型的条目
export type Category =
  | "movie"
  | "series"
  | "anime"
  | "variety"
  | "documentary"
  | "personal"
  | "other";

const CATEGORY_META: Record<Category, { label: string; mediaType: string | null }> = {
  movie: { label: "电影", mediaType: "Movie" },
  series: { label: "剧集", mediaType: "Series" },
  anime: { label: "动画", mediaType: "Anime" },
  variety: { label: "综艺", mediaType: "Variety" },
  documentary: { label: "纪录片", mediaType: "Documentary" },
  personal: { label: "个人", mediaType: "Personal" },
  other: { label: "其他", mediaType: null },
};

const KNOWN_TYPES = new Set(
  Object.values(CATEGORY_META)
    .map((m) => m.mediaType)
    .filter(Boolean),
);

export function isCategory(value: string): value is Category {
  return value in CATEGORY_META;
}

export default function Library({
  category,
  search,
  onOpen,
}: {
  category: Category;
  search: string;
  onOpen: (media: Media) => void;
}) {
  const meta = CATEGORY_META[category];
  const [media, setMedia] = useState<Media[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [scanning, setScanning] = useState(false);
  const [tag, setTag] = useState<string | null>(null);
  // 增量渲染:当前渲染条数,滚动到底再翻倍
  const [visibleCount, setVisibleCount] = useState(PAGE_SIZE);

  // 切换分类时重置 tag 筛选
  useEffect(() => {
    setTag(null);
  }, [category]);

  const load = async () => {
    setLoading(true);
    setError(null);
    try {
      setMedia(await api.getVideos());
    } catch (e) {
      console.error(e);
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    load();
  }, []);

  const onScan = async () => {
    setScanning(true);
    setError(null);
    try {
      const added = await api.scanVideos();
      toast.success(`扫描完成,新增 ${added} 个条目`);
      await load();
    } catch (e) {
      console.error(e);
      toast.error(`扫描失败: ${String(e)}`);
    } finally {
      setScanning(false);
    }
  };

  // 当前分类下出现过的标签,按条目数降序;忽略空标签(如根目录视频)
  const allTags = useMemo(() => {
    const counts = new Map<string, number>();
    for (const m of media) {
      const matchType = meta.mediaType
        ? m.media_type === meta.mediaType
        : !KNOWN_TYPES.has(m.media_type);
      if (!matchType) continue;
      for (const t of m.tags ?? []) {
        if (!t) continue;
        counts.set(t, (counts.get(t) ?? 0) + 1);
      }
    }
    return [...counts.entries()]
      .sort((a, b) => b[1] - a[1] || a[0].localeCompare(b[0]))
      .map(([t]) => t);
  }, [media, meta]);

  const filtered = media.filter((m) => {
    const kw = search.toLowerCase();
    const matchKeyword =
      !kw ||
      m.title.toLowerCase().includes(kw) ||
      m.file_path.toLowerCase().includes(kw);
    // other = 没有类型,或类型不属于上面任何分类
    const matchType = meta.mediaType
      ? m.media_type === meta.mediaType
      : !KNOWN_TYPES.has(m.media_type);
    const matchTag = tag === null || (m.tags ?? []).includes(tag);
    return matchKeyword && matchType && matchTag;
  });

  // 过滤条件/数据变化时重置增量渲染
  useEffect(() => {
    setVisibleCount(PAGE_SIZE);
  }, [search, category, tag, media]);

  // 哨兵:callback ref,节点一挂载就立即 observe,滚动接近底部自动加载下一批
  const sentinelRef = useRef<HTMLDivElement | null>(null);
  const observerRef = useRef<IntersectionObserver | null>(null);
  const loadMore = () => setVisibleCount((n) => n + PAGE_SIZE);

  useEffect(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting) loadMore();
      },
      { rootMargin: "400px" },
    );
    observerRef.current = observer;
    // 如果哨兵已挂载(比如过滤后重新渲染),立即观察
    if (sentinelRef.current) observer.observe(sentinelRef.current);
    return () => observer.disconnect();
  }, [filtered.length]);

  // callback ref:挂载即观察,卸载即断开,不依赖 effect 时机
  const setSentinel = (el: HTMLDivElement | null) => {
    sentinelRef.current = el;
    const obs = observerRef.current;
    if (obs) {
      obs.disconnect();
      if (el) obs.observe(el);
    }
  };

  const visible = filtered.slice(0, visibleCount);

  return (
    <div className="px-8 lg:px-10 py-8">
      <div className="flex items-center justify-between mb-5 animate-fade-in">
        <div>
          <h1 className="text-3xl font-bold text-strong tracking-tight">{meta.label}</h1>
          <p className="text-secondary text-sm mt-1">{filtered.length} 个条目</p>
        </div>
        <button
          onClick={onScan}
          disabled={scanning}
          className="px-5 py-2.5 rounded-full bg-orange-500 hover:bg-orange-400 active:scale-95 disabled:opacity-50 disabled:pointer-events-none text-white font-semibold text-sm shadow-lg shadow-orange-500/25 transition-all flex items-center gap-2"
        >
          {scanning ? (
            <>
              <Loader2 size={16} className="animate-spin" /> 扫描中…
            </>
          ) : (
            <>
              <RefreshCw size={15} /> 扫描
            </>
          )}
        </button>
      </div>

      {/* 标签筛选:与搜索叠加生效,当前分类没有标签时整行隐藏 */}
      {allTags.length > 0 && (
        <div className="flex flex-wrap w-fit max-w-full gap-1 p-1 mb-6 rounded-xl bg-tint/5 ring-1 ring-tint/10 animate-fade-in">
          {[null, ...allTags].map((t) => {
            const active = tag === t;
            return (
              <button
                key={t ?? "__all"}
                onClick={() => setTag(t)}
                className={`px-4 py-1.5 rounded-full text-sm transition-all duration-200 ${
                  active
                    ? "bg-tint/10 text-strong font-medium ring-1 ring-tint/10"
                    : "text-secondary hover:text-body"
                }`}
              >
                {t ?? "全部"}
              </button>
            );
          })}
        </div>
      )}

      {error && (
        <div className="text-err text-sm mb-6 bg-err/10 border border-err/20 rounded-xl px-4 py-3">
          {error}
        </div>
      )}

      {loading ? (
        <div className="flex flex-col items-center justify-center py-24 gap-3 text-mute">
          <Loader2 size={24} className="animate-spin" />
          <span className="text-sm">加载中…</span>
        </div>
      ) : filtered.length === 0 ? (
        <div className="flex flex-col items-center justify-center py-24 text-center animate-fade-in">
          <div className="w-20 h-20 rounded-2xl bg-tint/5 ring-1 ring-tint/10 flex items-center justify-center mb-6">
            <FolderOpen size={36} className="text-mute" />
          </div>
          <h2 className="text-xl font-semibold text-body mb-2">
            {media.length === 0 ? "媒体库是空的" : `「${meta.label}」分类下还没有内容`}
          </h2>
          <p className="text-secondary max-w-sm text-sm leading-relaxed">
            {media.length === 0
              ? "到「设置」里添加一个本地视频目录,然后回来点「扫描」。"
              : "换个标签、分类或搜索关键词试试。"}
          </p>
        </div>
      ) : (
        <>
          <div className={`${POSTER_GRID} animate-fade-in`}>
            {visible.map((m, i) => (
              <PosterCard key={m.id} media={m} index={i} onClick={() => onOpen(m)} />
            ))}
          </div>
          {/* 哨兵:始终在 DOM,滚动到接近底部时触发加载下一批 */}
          <div
            ref={setSentinel}
            className="h-10 flex items-center justify-center mt-6"
          >
            {visible.length < filtered.length && (
              <Loader2 size={20} className="animate-spin text-mute" />
            )}
          </div>
        </>
      )}
    </div>
  );
}
