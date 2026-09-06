import { useEffect, useState } from "react";
import { FolderOpen, Loader2, X } from "lucide-react";
import { api } from "../api";
import { toast } from "../toast";
import type { Config } from "../types";

export default function Settings() {
  const [config, setConfig] = useState<Config | null>(null);
  const [dirInput, setDirInput] = useState("");
  const [saving, setSaving] = useState(false);
  const [loading, setLoading] = useState(true);
  // 仅用于配置加载失败(页面级状态);保存结果走全局 toast
  const [loadError, setLoadError] = useState<string | null>(null);

  useEffect(() => {
    api
      .getConfig()
      .then(setConfig)
      .catch((e) => {
        console.error(e);
        setLoadError(String(e));
      })
      .finally(() => setLoading(false));
  }, []);

  const addDir = () => {
    if (!config || !dirInput.trim()) return;
    if (config.local_dirs.includes(dirInput.trim())) return;
    setConfig({ ...config, local_dirs: [...config.local_dirs, dirInput.trim()] });
    setDirInput("");
  };

  const removeDir = (dir: string) => {
    if (!config) return;
    setConfig({ ...config, local_dirs: config.local_dirs.filter((d) => d !== dir) });
  };

  const onSave = async () => {
    if (!config) return;
    setSaving(true);
    try {
      await api.saveConfig(config);
      toast.success("已保存到 config.toml");
    } catch (e) {
      console.error(e);
      toast.error(`保存失败: ${String(e)}`);
    } finally {
      setSaving(false);
    }
  };

  if (loading) {
    return (
      <div className="h-full flex flex-col items-center justify-center gap-3 text-mute">
        <Loader2 size={24} className="animate-spin" />
        <span className="text-sm">加载配置…</span>
      </div>
    );
  }

  if (!config) {
    return (
      <div className="px-8 lg:px-10 py-8">
        <h1 className="text-3xl font-bold text-strong tracking-tight mb-4">设置</h1>
        <div className="text-err">{loadError ?? "配置加载失败"}</div>
      </div>
    );
  }

  const inputClass =
    "w-full h-11 px-4 rounded-xl bg-tint/5 border border-tint/10 text-body placeholder:text-mute outline-none focus:border-orange-500/50 focus:ring-2 focus:ring-orange-500/20 transition-all";

  return (
    /* m-auto:内容区在页面中水平垂直居中,内容超高时自动退回顶部可滚动 */
    <div className="min-h-full flex px-6 lg:px-10 animate-fade-in">
      <div className="m-auto w-full max-w-2xl py-6">
        <h1 className="text-3xl font-bold text-strong tracking-tight mb-8 text-center">
          设置
        </h1>

        <div className="space-y-5">
          {/* 本地视频目录 */}
          <section className="rounded-2xl bg-tint/3 ring-1 ring-tint/10 p-5">
            <h2 className="text-strong font-semibold mb-1">本地视频目录</h2>
            <p className="text-secondary text-xs mb-5">
              添加包含视频文件的文件夹,扫描时会递归查找。
            </p>

            <div className="space-y-2 mb-4">
              {config.local_dirs.length === 0 ? (
                <p className="text-secondary text-sm">还没有目录,添加一个吧。</p>
              ) : (
                config.local_dirs.map((dir) => (
                  <div
                    key={dir}
                    className="group flex items-center gap-3 px-4 py-2.5 rounded-xl bg-tint/5 ring-1 ring-tint/5 hover:ring-tint/20 transition-all"
                  >
                    <FolderOpen size={15} className="text-mute shrink-0" />
                    <span className="text-body text-sm font-mono truncate flex-1">
                      {dir}
                    </span>
                    <button
                      onClick={() => removeDir(dir)}
                      className="text-mute hover:text-err opacity-0 group-hover:opacity-100 transition-all"
                      aria-label={`移除 ${dir}`}
                    >
                      <X size={16} />
                    </button>
                  </div>
                ))
              )}
            </div>

            <div className="flex gap-2">
              <input
                value={dirInput}
                onChange={(e) => setDirInput(e.target.value)}
                onKeyDown={(e) => e.key === "Enter" && addDir()}
                placeholder="例如 D:\Videos\Movies"
                className={`flex-1 ${inputClass}`}
              />
              <button
                onClick={addDir}
                className="px-5 rounded-xl bg-tint/10 hover:bg-tint/15 text-strong font-medium text-sm transition-colors"
              >
                添加
              </button>
            </div>
          </section>

          {/* 播放器 */}
          <section className="rounded-2xl bg-tint/3 ring-1 ring-tint/10 p-5">
            <h2 className="text-strong font-semibold mb-1">播放器</h2>
            <p className="text-secondary text-xs mb-5">留空则使用系统默认播放器打开。</p>
            <input
              value={config.player_name ?? ""}
              onChange={(e) =>
                setConfig({ ...config, player_name: e.target.value || null })
              }
              placeholder="例如 mpv / VLC / PotPlayer"
              className={inputClass}
            />
          </section>

          {/* 封面缓存 */}
          <section className="rounded-2xl bg-tint/3 ring-1 ring-tint/10 p-5">
            <h2 className="text-strong font-semibold mb-1">封面缓存</h2>
            <p className="text-secondary text-xs mb-4">海报缩略图缓存目录。</p>
            <p className="text-body text-sm font-mono px-4 py-2.5 rounded-xl bg-tint/5 ring-1 ring-tint/5">
              ~/.mona/posters
            </p>
            <p className="text-mute text-xs mt-3">
              注意: 封面缓存目录目前由后端固定,后续如需可配置,可在 config.toml 中扩展字段。
            </p>
          </section>

          <button
            onClick={onSave}
            disabled={saving}
            className="w-full py-3 rounded-xl bg-orange-500 hover:bg-orange-400 active:scale-[0.98] disabled:opacity-50 text-white font-semibold shadow-lg shadow-orange-500/25 transition-all"
          >
            {saving ? "保存中…" : "保存设置"}
          </button>
        </div>
      </div>
    </div>
  );
}
