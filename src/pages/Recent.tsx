import { useEffect, useState } from "react";
import { History, Loader2 } from "lucide-react";
import { api } from "../api";
import { POSTER_GRID, PosterCard } from "../components/MediaCard";
import type { Media } from "../types";

export default function Recent({
  search,
  onOpen,
  refreshKey,
}: {
  search: string;
  onOpen: (media: Media) => void;
  refreshKey: number;
}) {
  const [recent, setRecent] = useState<Media[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    setLoading(true);
    api
      .getRecentPlayed()
      .then(setRecent)
      .catch((e) => {
        console.error(e);
        setError(String(e));
      })
      .finally(() => setLoading(false));
  }, [refreshKey]);

  const kw = search.toLowerCase();
  const filtered = recent.filter(
    (m) =>
      !kw ||
      m.title.toLowerCase().includes(kw) ||
      m.file_path.toLowerCase().includes(kw),
  );

  return (
    <div className="px-8 lg:px-10 py-8">
      <header className="mb-8 animate-fade-in">
        <h1 className="text-3xl font-bold text-strong tracking-tight">最近观看</h1>
        {!loading && filtered.length > 0 && (
          <p className="text-secondary text-sm mt-1">{filtered.length} 个条目</p>
        )}
      </header>

      {error && (
        <div className="text-err text-sm mb-6 bg-err/10 border border-err/20 rounded-xl px-4 py-3">
          加载失败: {error}
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
            <History size={36} className="text-mute" />
          </div>
          <h2 className="text-xl font-semibold text-body mb-2">还没有观看记录</h2>
          <p className="text-secondary max-w-sm text-sm leading-relaxed">
            播放过的视频会出现在这里。
          </p>
        </div>
      ) : (
        <div className={POSTER_GRID}>
          {filtered.map((m, i) => (
            <PosterCard key={m.id} media={m} index={i} onClick={() => onOpen(m)} />
          ))}
        </div>
      )}
    </div>
  );
}
