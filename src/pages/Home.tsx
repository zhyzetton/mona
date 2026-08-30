import { useEffect, useState } from "react";
import { Film, LayoutGrid } from "lucide-react";
import { api, assetURL } from "../api";
import { PosterCard } from "../components/MediaCard";
import type { Media } from "../types";

export default function Home({
  search,
  onOpen,
  refreshKey,
}: {
  search: string;
  onOpen: (media: Media) => void;
  refreshKey: number;
}) {
  const [media, setMedia] = useState<Media[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    api
      .getVideos()
      .then(setMedia)
      .catch((e) => {
        console.error(e);
        setError(String(e));
      });
  }, [refreshKey]);

  const kw = search.toLowerCase();
  const filtered = media.filter(
    (m) =>
      !kw ||
      m.title.toLowerCase().includes(kw) ||
      m.file_path.toLowerCase().includes(kw),
  );

  const watch = filtered.filter((m) => m.poster_path).slice(0, 3);
  const recent = filtered.slice(0, 10);

  return (
    <div className="px-8 py-6">
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold text-slate-900">主屏幕</h1>
        <LayoutGrid size={20} className="text-slate-400" />
      </div>

      {error && (
        <div className="text-red-500 text-sm mb-6 bg-red-50 border border-red-200 rounded-xl px-4 py-3">
          加载失败: {error}
        </div>
      )}

      {/* 正在观看: 大图横幅 */}
      {watch.length > 0 && (
        <section className="mb-10">
          <h2 className="text-lg font-semibold text-slate-900 mb-4">正在观看</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {watch.map((m) => (
              <div
                key={m.id}
                className="group cursor-pointer"
                onClick={() => onOpen(m)}
              >
                <div className="aspect-video rounded-2xl overflow-hidden bg-slate-100 shadow-sm group-hover:shadow-md transition-all">
                  <img
                    src={assetURL(m.poster_path)}
                    alt={m.title}
                    className="w-full h-full object-cover"
                  />
                </div>
                <p className="mt-2 text-sm font-medium text-slate-800">{m.title}</p>
              </div>
            ))}
          </div>
        </section>
      )}

      {/* 最近添加 */}
      {media.length === 0 && !error ? (
        <div className="flex flex-col items-center justify-center py-24 text-center">
          <Film size={56} className="text-slate-300 mb-6" />
          <h2 className="text-xl font-semibold text-slate-800 mb-2">还没有媒体</h2>
          <p className="text-slate-500 max-w-sm">
            去「设置」添加一个本地视频目录,然后到「媒体库」扫描。
          </p>
        </div>
      ) : (
        <section>
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-lg font-semibold text-slate-900">最近添加</h2>
            <button className="text-sm text-orange-500 hover:text-orange-600">
              查看全部
            </button>
          </div>
          <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-x-4 gap-y-6">
            {recent.map((m) => (
              <PosterCard key={m.id} media={m} onClick={() => onOpen(m)} />
            ))}
          </div>
        </section>
      )}
    </div>
  );
}
