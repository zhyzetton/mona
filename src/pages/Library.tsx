import { useEffect, useRef, useState } from "react";
import { FolderOpen, Loader2, X } from "lucide-react";
import { api } from "../api";
import { PosterCard } from "../components/MediaCard";
import type { Media } from "../types";

const PAGE_SIZE = 30;

export default function Library({
  search,
  onOpen,
}: {
  search: string;
  onOpen: (media: Media) => void;
}) {
  const [media, setMedia] = useState<Media[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [scanning, setScanning] = useState(false);
  const [filter, setFilter] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  // 增量渲染:当前渲染条数,滚动到底再翻倍
  const [visibleCount, setVisibleCount] = useState(PAGE_SIZE);

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
    setNotice(null);
    try {
      const added = await api.scanVideos();
      setNotice(`扫描完成,新增 ${added} 个条目`);
      await load();
    } catch (e) {
      console.error(e);
      setError(String(e));
    } finally {
      setScanning(false);
    }
  };

  const filtered = media.filter((m) => {
    const kw = search.toLowerCase();
    const matchKeyword =
      !kw ||
      m.title.toLowerCase().includes(kw) ||
      m.file_path.toLowerCase().includes(kw);
    const matchType = filter === null || m.media_type === filter;
    return matchKeyword && matchType;
  });

  // 过滤条件/数据变化时重置增量渲染
  useEffect(() => {
    setVisibleCount(PAGE_SIZE);
  }, [search, filter, media]);

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
    <div className="px-8 py-6">
      <div className="flex items-center justify-between mb-4">
        <div>
          <h1 className="text-2xl font-bold text-slate-900">媒体库</h1>
          <p className="text-slate-400 text-sm">{media.length} 个条目</p>
        </div>
        <button
          onClick={onScan}
          disabled={scanning}
          className="px-4 py-2 rounded-xl bg-orange-500 hover:bg-orange-600 disabled:opacity-50 text-white font-semibold text-sm transition-colors flex items-center gap-2"
        >
          {scanning ? (
            <>
              <Loader2 size={16} className="animate-spin" /> 扫描中…
            </>
          ) : (
            <>扫描</>
          )}
        </button>
      </div>

      {/* 扫描通知 */}
      {notice && (
        <div className="mb-4 px-4 py-3 rounded-xl bg-green-50 border border-green-200 text-green-600 text-sm flex items-center justify-between">
          <span>✓ {notice}</span>
          <button
            onClick={() => setNotice(null)}
            className="text-green-400 hover:text-green-600"
          >
            <X size={16} />
          </button>
        </div>
      )}

      {/* 分类筛选 */}
      <div className="flex gap-2 mb-6">
        <button
          onClick={() => setFilter(null)}
          className={`px-3 py-1.5 rounded-full text-sm transition-colors ${
            filter === null
              ? "bg-orange-500 text-white font-semibold"
              : "bg-white text-slate-600 hover:bg-slate-100"
          }`}
        >
          全部
        </button>
        {[
          { key: "Movie", label: "电影" },
          { key: "Series", label: "剧集" },
          { key: "Anime", label: "动画" },
          { key: "Local", label: "本地" },
        ].map((opt) => (
          <button
            key={opt.key}
            onClick={() => setFilter(opt.key)}
            className={`px-3 py-1.5 rounded-full text-sm transition-colors ${
              filter === opt.key
                ? "bg-orange-500 text-white font-semibold"
                : "bg-white text-slate-600 hover:bg-slate-100"
            }`}
          >
            {opt.label}
          </button>
        ))}
      </div>

      {error && (
        <div className="text-red-500 text-sm mb-6 bg-red-50 border border-red-200 rounded-xl px-4 py-3">
          {error}
        </div>
      )}

      {loading ? (
        <div className="flex justify-center py-24 text-slate-400">加载中…</div>
      ) : filtered.length === 0 ? (
        <div className="flex flex-col items-center justify-center py-24 text-center">
          <FolderOpen size={56} className="text-slate-300 mb-6" />
          <h2 className="text-xl font-semibold text-slate-800 mb-2">
            {media.length === 0 ? "媒体库是空的" : "没有匹配的结果"}
          </h2>
          <p className="text-slate-500 max-w-sm">
            {media.length === 0
              ? "到「设置」里添加一个本地视频目录,然后回来点「扫描」。"
              : "换个关键词或筛选条件试试。"}
          </p>
        </div>
      ) : (
        <>
          <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-x-4 gap-y-6">
            {visible.map((m) => (
              <PosterCard key={m.id} media={m} onClick={() => onOpen(m)} />
            ))}
          </div>
          {/* 哨兵:始终在 DOM,滚动到接近底部时触发加载下一批 */}
          <div
            ref={setSentinel}
            className="h-10 flex items-center justify-center mt-4"
          >
            {visible.length < filtered.length && (
              <Loader2 size={20} className="animate-spin text-slate-300" />
            )}
          </div>
        </>
      )}
    </div>
  );
}
