import { useState } from "react";
import {
  Calendar,
  ChevronDown,
  ChevronLeft,
  Clock,
  Play,
  User,
} from "lucide-react";
import { api, assetURL, canPlayInWebview } from "../api";
import { mediaTypeLabel } from "../components/MediaCard";
import type { Media } from "../types";

// 常见的第三方播放器名称
const EXTERNAL_PLAYERS = ["PotPlayer", "mpv", "VLC", "Windows Media Player"];

export default function Detail({
  media,
  onBack,
  onPlay,
}: {
  media: Media;
  onBack: () => void;
  onPlay: (media: Media) => void;
}) {
  const [error, setError] = useState<string | null>(null);
  const [showFull, setShowFull] = useState(false);
  const [menuOpen, setMenuOpen] = useState(false);

  // 主播放按钮:能内嵌播放的进独立播放页,否则回退系统播放器
  const playInApp = () => {
    setError(null);
    if (canPlayInWebview(media.file_path)) {
      onPlay(media);
    } else {
      playExternally();
    }
  };

  const playExternally = async () => {
    setError(null);
    try {
      await api.playVideo(media.id!);
    } catch (e) {
      console.error(e);
      setError(String(e));
    }
  };

  const playWithPlayer = async (player: string) => {
    setMenuOpen(false);
    setError(null);
    try {
      await api.playVideoWithPlayer(media.id!, player);
    } catch (e) {
      console.error(e);
      setError(String(e));
    }
  };

  // 信息字段"暂无"兜底
  const year = media.year ?? "暂无";
  const rating = media.rating !== null ? `★ ${media.rating}` : "暂无";
  const overview = media.overview || "暂无简介";
  const actors = media.actors.length > 0 ? media.actors : ["暂无演员"];
  const type = mediaTypeLabel(media.media_type);

  const cover = assetURL(media.detail_img_path || media.poster_path);

  return (
    <div className="min-h-screen bg-[#f2f3f5]">
      {/* 顶部沉浸式大图:占视口一半以上高度,海报图作为背景 */}
      <div className="relative h-[58vh] min-h-[440px]">
        <div className="absolute inset-0 z-0 overflow-hidden">
          {cover ? (
            <img
              src={cover}
              alt=""
              className="w-full h-full object-cover object-center"
            />
          ) : (
            <div className="w-full h-full bg-gradient-to-b from-slate-800 to-slate-900" />
          )}
          {/* 顶部压暗,让返回箭头清晰 */}
          <div className="absolute inset-x-0 top-0 h-24 bg-gradient-to-b from-black/50 to-transparent" />
          {/* 底部沉浸式模糊过渡:
              1) blur 层用 mask 渐变,让模糊从无到有平滑出现(避免一刀切的分界线)
              2) 下面再压一层颜色淡出渐变,让海报颜色融入浅色内容区 */}
          <div
            className="absolute inset-x-0 bottom-0 h-48"
            style={{
              backdropFilter: "blur(20px)",
              WebkitBackdropFilter: "blur(20px)",
              maskImage: "linear-gradient(to bottom, transparent, black)",
              WebkitMaskImage: "linear-gradient(to bottom, transparent, black)",
            }}
          />
          <div className="absolute inset-x-0 bottom-0 h-40 bg-gradient-to-b from-transparent to-[#f2f3f5]" />
        </div>

        {/* 返回按钮:最高层,任何时候都可点击 */}
        <button
          onClick={onBack}
          className="absolute top-3 left-4 z-50 p-2.5 rounded-full text-white/90 hover:text-white hover:bg-white/15 cursor-pointer transition-colors"
          aria-label="返回"
        >
          <ChevronLeft size={28} />
        </button>

        {/* 内容层:靠底部对齐 */}
        <div className="relative z-10 h-full flex flex-col justify-end px-8 pb-10">
          <h1 className="text-4xl font-bold text-white drop-shadow-[0_2px_8px_rgba(0,0,0,0.7)] mb-4">
            {media.title}
          </h1>

          <div className="flex flex-wrap items-center gap-4">
            {/* 播放 + 下拉(严格等高) */}
            <div className="relative flex items-stretch">
              <button
                onClick={playInApp}
                className="flex items-center gap-3 px-8 h-12 rounded-l-lg bg-white text-slate-900 font-semibold hover:bg-white/90 transition-colors"
              >
                <span className="text-lg leading-none">
                  <Play size={20} />
                </span>
                {canPlayInWebview(media.file_path) ? "播放" : "用系统播放器播放"}
              </button>
              <button
                onClick={() => setMenuOpen((v) => !v)}
                className="flex items-center px-3 h-12 rounded-r-lg bg-white text-slate-900 border-l border-slate-200 hover:bg-white/90 transition-colors"
                aria-label="选择播放器"
              >
                <span className="leading-none">
                  <ChevronDown size={16} />
                </span>
              </button>

              {menuOpen && (
                <>
                  <div
                    className="fixed inset-0 z-40"
                    onClick={() => setMenuOpen(false)}
                  />
                  <div className="absolute top-full right-0 z-50 mt-2 w-52 bg-white rounded-xl shadow-xl border border-slate-200 py-1 overflow-hidden">
                    <div className="px-4 pt-2 pb-1 text-xs text-slate-400">
                      使用第三方播放器
                    </div>
                    {EXTERNAL_PLAYERS.map((p) => (
                      <button
                        key={p}
                        onClick={() => playWithPlayer(p)}
                        className="w-full text-left px-4 py-2 text-sm text-slate-700 hover:bg-orange-50 hover:text-orange-600 transition-colors"
                      >
                        {p}
                      </button>
                    ))}
                  </div>
                </>
              )}
            </div>

            {/* 元信息 */}
            <div className="text-white text-sm space-y-1 drop-shadow">
              <div className="flex items-center gap-4">
                <span className="text-green-400 font-semibold">{rating}</span>
                <span className="flex items-center gap-1"><Calendar size={14} /> {year}</span>
                <span className="flex items-center gap-1"><Clock size={14} /> {media.duration ?? "暂无"}</span>
              </div>
              <div className="text-white/80 text-xs">
                {type} · {media.resolution ? `${media.resolution}p` : "暂无清晰度"} · {media.file_size || "暂无大小"}
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* 下方信息区 */}
      <div className="px-8 pb-8">
        {error && (
          <div className="mb-6 text-red-500 text-sm bg-red-50 border border-red-200 rounded-xl px-4 py-3">
            {error}
          </div>
        )}

        {/* 剧情简介 */}
        <section className="mb-6">
          <h2 className="text-lg font-semibold text-slate-900 mb-2">剧情简介</h2>
          <p className="text-slate-600 leading-relaxed text-sm">
            {showFull ? overview : `${overview.slice(0, 120)}${overview.length > 120 ? "…" : ""}`}
            {overview.length > 120 && (
              <button
                onClick={() => setShowFull((v) => !v)}
                className="text-orange-500 hover:text-orange-600 ml-1"
              >
                {showFull ? "收起" : "全部"}
              </button>
            )}
          </p>
        </section>

        {/* 相关演员 */}
        <section className="mb-6">
          <h2 className="text-lg font-semibold text-slate-900 mb-3">相关演员</h2>
          <div className="flex gap-3 overflow-x-auto pb-2">
            {actors.map((name, i) => (
              <div key={i} className="flex flex-col items-center shrink-0 w-20 text-center">
                <div className="w-16 h-16 aspect-square rounded-full bg-gradient-to-br from-slate-200 to-slate-300 flex items-center justify-center text-slate-400 text-2xl overflow-hidden">
                  {media.actors.length > 0 ? (
                    <span>{name.slice(0, 1)}</span>
                  ) : (
                    <User size={24} />
                  )}
                </div>
                <p className="mt-2 text-xs text-slate-700 truncate w-full">{name}</p>
                {media.actors.length > 0 && (
                  <p className="text-[10px] text-slate-400 truncate w-full">演员</p>
                )}
              </div>
            ))}
          </div>
        </section>

        {/* 文件信息 */}
        <section className="mb-6">
          <h2 className="text-lg font-semibold text-slate-900 mb-2">文件信息</h2>
          <div className="bg-white rounded-2xl p-5 shadow-sm space-y-3 text-sm">
            <div className="flex">
              <span className="text-slate-400 w-24 shrink-0">文件名</span>
              <span className="text-slate-700 break-all">{media.file_path.split(/[\\/]/).pop() ?? "暂无"}</span>
            </div>
            <div className="flex">
              <span className="text-slate-400 w-24 shrink-0">路径</span>
              <span className="text-slate-700 break-all">{media.file_path || "暂无"}</span>
            </div>
            <div className="flex">
              <span className="text-slate-400 w-24 shrink-0">时长</span>
              <span className="text-slate-700">{media.duration ?? "暂无"}</span>
            </div>
            <div className="flex">
              <span className="text-slate-400 w-24 shrink-0">大小</span>
              <span className="text-slate-700">{media.file_size || "暂无"}</span>
            </div>
          </div>
        </section>
      </div>
    </div>
  );
}
