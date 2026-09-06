import { useEffect, useState } from "react";
import type { ReactNode } from "react";
import {
  Calendar,
  ChevronDown,
  ChevronLeft,
  Clock,
  Play,
  Star,
  User,
} from "lucide-react";
import { api, assetURL, canPlayInWebview } from "../api";
import { getDominantColor } from "../color";
import { mediaTypeLabel } from "../components/MediaCard";
import { toast } from "../toast";
import type { Media } from "../types";

// 常见的第三方播放器名称
const EXTERNAL_PLAYERS = ["PotPlayer", "mpv", "VLC", "Windows Media Player"];

// 渐进模糊分层:每层用 backdrop-filter 给身后画面加 blur,蒙版软过渡避免层间接缝。
// 越往下覆盖的层数越多,累计模糊越强,模拟连续景深;数值可按观感增删层数/强度
const PROGRESSIVE_BLURS = [
  { top: "42%", blur: 3, ramp: 45 },
  { top: "54%", blur: 8, ramp: 40 },
  { top: "66%", blur: 18, ramp: 40 },
  { top: "78%", blur: 40, ramp: 40 },
];

// 元信息小徽章(位于 hero 深色渐变之上,两种主题下都是白字深底)
function Chip({ children, accent }: { children: ReactNode; accent?: boolean }) {
  return (
    <span
      className={`flex items-center gap-1.5 px-2.5 py-1 rounded-md bg-white/10 backdrop-blur-sm text-xs ring-1 ring-white/15 ${
        accent ? "text-orange-300 font-semibold" : "text-zinc-100"
      }`}
    >
      {children}
    </span>
  );
}

export default function Detail({
  media,
  onBack,
  onPlay,
}: {
  media: Media;
  onBack: () => void;
  onPlay: (media: Media) => void;
}) {
  const [showFull, setShowFull] = useState(false);
  const [menuOpen, setMenuOpen] = useState(false);
  const cover = assetURL(media.detail_img_path || media.poster_path);
  // 主色:hero 图加载后提取,用于氛围光与整页底色;提取失败时保持原样
  const [accent, setAccent] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    setAccent(null);
    if (cover) {
      getDominantColor(cover).then((c) => {
        if (!cancelled) setAccent(c);
      });
    }
    return () => {
      cancelled = true;
    };
  }, [cover]);

  // 记录一次播放到后端;失败只打日志,不阻塞播放
  const recordPlay = () => {
    if (media.id == null) return;
    api.recordPlay(media.id).catch((e) => console.error(e));
  };

  // 主播放按钮:能内嵌播放的进独立播放页,否则回退系统播放器
  const playInApp = () => {
    recordPlay();
    if (canPlayInWebview(media.file_path)) {
      onPlay(media);
    } else {
      playExternally();
    }
  };

  const playExternally = async () => {
    try {
      await api.playVideo(media.id!);
    } catch (e) {
      console.error(e);
      toast.error(`播放失败: ${String(e)}`);
    }
  };

  const playWithPlayer = async (player: string) => {
    setMenuOpen(false);
    recordPlay();
    try {
      await api.playVideoWithPlayer(media.id!, player);
    } catch (e) {
      console.error(e);
      toast.error(`播放失败: ${String(e)}`);
    }
  };

  const overview = media.overview || "暂无简介";
  const actors = media.actors.length > 0 ? media.actors : ["暂无演员"];
  // 页面底色:有主色时轻微染色。底部渐隐的终点必须与它完全一致才能无缝
  const pageBg = accent
    ? `color-mix(in srgb, ${accent} 14%, var(--c-base))`
    : "var(--c-base)";

  return (
    <div
      className="min-h-full bg-base transition-colors duration-700"
      style={{ backgroundColor: pageBg }}
    >
      {/* 顶部沉浸式大图:海报铺满,底部压暗后渐隐融入页面底色 */}
      <div className="relative h-[62vh] min-h-[480px]">
        <div className="absolute inset-0 z-0 overflow-hidden">
          {cover ? (
            <img
              src={cover}
              alt=""
              className="w-full h-full object-cover object-top animate-hero-in"
            />
          ) : (
            <div className="w-full h-full bg-gradient-to-b from-tint/20 to-base" />
          )}

          {/* 渐进模糊:海报上部保持清晰,从 42% 高度起越往下越模糊,
              底边处已虚化成色晕,与页面底色衔接后没有分界线 */}
          {PROGRESSIVE_BLURS.map(({ top, blur, ramp }) => (
            <div
              key={blur}
              className="absolute inset-x-0 bottom-0"
              style={{
                top,
                backdropFilter: `blur(${blur}px)`,
                WebkitBackdropFilter: `blur(${blur}px)`,
                maskImage: `linear-gradient(to bottom, transparent, black ${ramp}%)`,
                WebkitMaskImage: `linear-gradient(to bottom, transparent, black ${ramp}%)`,
              }}
            />
          ))}

          {/* 主色氛围光:从底部往上染一层,主色提取完成后淡入 */}
          {accent && (
            <div
              className="absolute inset-0 animate-fade-in"
              style={{
                background: `linear-gradient(to top, ${accent}66, transparent 60%)`,
              }}
            />
          )}

          {/* 顶部压暗,让返回箭头清晰(浅一点,少吞海报顶部) */}
          <div className="absolute inset-x-0 top-0 h-28 bg-gradient-to-b from-black/45 to-transparent" />
          {/* 底部文字压暗区:标题/徽章/按钮全部位于此区内,保证白字可读 */}
          <div className="absolute inset-x-0 bottom-0 h-2/3 bg-gradient-to-t from-black/65 via-black/25 to-transparent" />
          {/* 底色渐隐兜底:图片蒙版负责消除边界,这层只负责在 88% 前过渡到
              页面底色(保证浅色主题下白字可读、底边颜色严格一致),92% 后保持不动 */}
          <div
            className="absolute inset-x-0 bottom-0 h-[45%]"
            style={{
              background: `linear-gradient(to bottom, transparent, color-mix(in srgb, ${pageBg} 20%, transparent) 30%, color-mix(in srgb, ${pageBg} 55%, transparent) 60%, ${pageBg} 88%)`,
            }}
          />
        </div>

        {/* 返回按钮:最高层,任何时候都可点击 */}
        <button
          onClick={onBack}
          className="absolute top-4 left-5 z-50 p-2.5 rounded-full bg-black/30 backdrop-blur-md ring-1 ring-white/10 text-white/90 hover:text-white hover:bg-black/50 hover:scale-105 active:scale-95 cursor-pointer transition-all"
          aria-label="返回"
        >
          <ChevronLeft size={24} />
        </button>

        {/* 内容层:靠底部对齐 */}
        <div className="relative z-10 h-full flex flex-col justify-end px-8 lg:px-12 pb-14">
          <h1 className="text-4xl lg:text-5xl font-bold text-white tracking-tight drop-shadow-[0_2px_12px_rgba(0,0,0,0.8)] mb-5 animate-fade-up">
            {media.title}
          </h1>

          {/* 元信息徽章行 */}
          <div
            className="flex flex-wrap items-center gap-2 mb-6 animate-fade-up"
            style={{ animationDelay: "60ms" }}
          >
            {media.rating !== null && (
              <Chip accent>
                <Star size={12} fill="currentColor" /> {media.rating}
              </Chip>
            )}
            {media.year && (
              <Chip>
                <Calendar size={12} /> {media.year}
              </Chip>
            )}
            {media.duration && (
              <Chip>
                <Clock size={12} /> {media.duration}
              </Chip>
            )}
            <Chip>{mediaTypeLabel(media.media_type)}</Chip>
            {media.resolution > 0 && <Chip>{media.resolution}p</Chip>}
            {media.file_size && <Chip>{media.file_size}</Chip>}
          </div>

          {/* 播放 + 下拉(严格等高) */}
          <div
            className="relative flex items-stretch w-fit animate-fade-up"
            style={{ animationDelay: "120ms" }}
          >
            <button
              onClick={playInApp}
              className="flex items-center gap-2.5 pl-7 pr-6 h-12 rounded-l-full bg-orange-500 hover:bg-orange-400 text-white font-semibold shadow-lg shadow-orange-500/30 transition-all hover:scale-[1.03] active:scale-95"
            >
              <Play size={18} fill="currentColor" />
              {canPlayInWebview(media.file_path) ? "播放" : "用系统播放器播放"}
            </button>
            <button
              onClick={() => setMenuOpen((v) => !v)}
              className="flex items-center px-3.5 h-12 rounded-r-full bg-orange-500 hover:bg-orange-400 text-white border-l border-black/20 shadow-lg shadow-orange-500/30 transition-all"
              aria-label="选择播放器"
            >
              <ChevronDown
                size={16}
                className={`transition-transform duration-200 ${menuOpen ? "rotate-180" : ""}`}
              />
            </button>

            {menuOpen && (
              <>
                <div
                  className="fixed inset-0 z-40"
                  onClick={() => setMenuOpen(false)}
                />
                <div className="absolute top-full right-0 z-50 mt-2 w-56 glass rounded-xl shadow-2xl py-1.5 overflow-hidden animate-scale-in origin-top-right">
                  <div className="px-4 pt-2 pb-1.5 text-xs text-mute">
                    使用第三方播放器
                  </div>
                  {EXTERNAL_PLAYERS.map((p) => (
                    <button
                      key={p}
                      onClick={() => playWithPlayer(p)}
                      className="w-full text-left px-4 py-2 text-sm text-body hover:bg-tint/5 hover:text-strong transition-colors"
                    >
                      {p}
                    </button>
                  ))}
                </div>
              </>
            )}
          </div>
        </div>
      </div>

      {/* 下方信息区 */}
      <div className="px-8 lg:px-12 pb-12 space-y-10">
        {/* 剧情简介 */}
        <section>
          <h2 className="text-lg font-semibold text-strong mb-3">剧情简介</h2>
          <p className="text-secondary leading-relaxed text-sm max-w-3xl">
            {showFull ? overview : `${overview.slice(0, 120)}${overview.length > 120 ? "…" : ""}`}
            {overview.length > 120 && (
              <button
                onClick={() => setShowFull((v) => !v)}
                className="text-orange-500 hover:text-orange-400 ml-1 transition-colors"
              >
                {showFull ? "收起" : "全部"}
              </button>
            )}
          </p>
        </section>

        {/* 相关演员 */}
        <section>
          <h2 className="text-lg font-semibold text-strong mb-4">相关演员</h2>
          <div className="flex gap-5 overflow-x-auto no-scrollbar pb-2">
            {actors.map((name, i) => (
              <div key={i} className="flex flex-col items-center shrink-0 w-20 text-center">
                <div className="w-16 h-16 aspect-square rounded-full bg-tint/10 ring-1 ring-tint/10 flex items-center justify-center text-secondary text-xl font-medium overflow-hidden">
                  {media.actors.length > 0 ? (
                    <span>{name.slice(0, 1)}</span>
                  ) : (
                    <User size={24} />
                  )}
                </div>
                <p className="mt-2.5 text-xs text-body truncate w-full">{name}</p>
                {media.actors.length > 0 && (
                  <p className="text-[10px] text-mute truncate w-full">演员</p>
                )}
              </div>
            ))}
          </div>
        </section>

        {/* 文件信息 */}
        <section>
          <h2 className="text-lg font-semibold text-strong mb-3">文件信息</h2>
          <div className="rounded-2xl bg-tint/3 ring-1 ring-tint/10 px-6 py-2 text-sm">
            {[
              { label: "文件名", value: media.file_path.split(/[\\/]/).pop() },
              { label: "路径", value: media.file_path },
              { label: "标签", value: (media.tags ?? []).join("、") || null },
              { label: "时长", value: media.duration },
              { label: "大小", value: media.file_size },
            ].map((row) => (
              <div
                key={row.label}
                className="flex gap-4 py-3 border-b border-tint/5 last:border-0"
              >
                <span className="text-mute w-16 shrink-0">{row.label}</span>
                <span className="text-body break-all">{row.value || "暂无"}</span>
              </div>
            ))}
          </div>
        </section>
      </div>
    </div>
  );
}
