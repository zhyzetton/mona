import { useEffect, useState } from "react";
import { X } from "lucide-react";
import { api } from "../api";
import type { Config } from "../types";

export default function Settings() {
  const [config, setConfig] = useState<Config | null>(null);
  const [dirInput, setDirInput] = useState("");
  const [saving, setSaving] = useState(false);
  const [loading, setLoading] = useState(true);
  const [message, setMessage] = useState<{ type: "ok" | "error"; text: string } | null>(null);

  useEffect(() => {
    api
      .getConfig()
      .then(setConfig)
      .catch((e) => {
        console.error(e);
        setMessage({ type: "error", text: String(e) });
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
    setMessage(null);
    try {
      await api.saveConfig(config);
      setMessage({ type: "ok", text: "已保存到 config.toml" });
    } catch (e) {
      console.error(e);
      setMessage({ type: "error", text: String(e) });
    } finally {
      setSaving(false);
    }
  };

  if (loading) {
    return <div className="flex justify-center py-24 text-slate-400">加载配置…</div>;
  }

  if (!config) {
    return (
      <div className="px-8 py-6">
        <h1 className="text-2xl font-bold text-slate-900 mb-4">设置</h1>
        <div className="text-red-500">{message?.text ?? "配置加载失败"}</div>
      </div>
    );
  }

  return (
    <div className="px-8 py-6 max-w-2xl">
      <h1 className="text-2xl font-bold text-slate-900 mb-6">设置</h1>

      <div className="space-y-6">
        {/* 本地视频目录 */}
        <section className="bg-white rounded-2xl p-5 shadow-sm">
          <h2 className="text-slate-900 font-semibold mb-1">本地视频目录</h2>
          <p className="text-slate-400 text-xs mb-4">
            添加包含视频文件的文件夹,扫描时会递归查找。
          </p>

          <div className="space-y-2 mb-4">
            {config.local_dirs.length === 0 ? (
              <p className="text-slate-400 text-sm">还没有目录,添加一个吧。</p>
            ) : (
              config.local_dirs.map((dir) => (
                <div
                  key={dir}
                  className="flex items-center justify-between bg-slate-50 rounded-lg px-4 py-2.5"
                >
                  <span className="text-slate-700 text-sm font-mono truncate flex-1">{dir}</span>
                  <button
                    onClick={() => removeDir(dir)}
                    className="ml-3 text-slate-300 hover:text-red-500 transition-colors"
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
              className="flex-1 px-4 py-2.5 rounded-xl bg-slate-50 border border-slate-200 text-slate-700 placeholder-slate-400 outline-none focus:border-orange-400/60 transition-colors"
            />
            <button
              onClick={addDir}
              className="px-4 py-2.5 rounded-xl bg-slate-100 hover:bg-slate-200 text-slate-700 font-semibold text-sm transition-colors"
            >
              添加
            </button>
          </div>
        </section>

        {/* 播放器 */}
        <section className="bg-white rounded-2xl p-5 shadow-sm">
          <h2 className="text-slate-900 font-semibold mb-1">播放器</h2>
          <p className="text-slate-400 text-xs mb-4">留空则使用系统默认播放器打开。</p>
          <input
            value={config.player_name ?? ""}
            onChange={(e) =>
              setConfig({ ...config, player_name: e.target.value || null })
            }
            placeholder="例如 mpv / VLC / PotPlayer"
            className="w-full px-4 py-2.5 rounded-xl bg-slate-50 border border-slate-200 text-slate-700 placeholder-slate-400 outline-none focus:border-orange-400/60 transition-colors"
          />
        </section>

        {/* 封面缓存 */}
        <section className="bg-white rounded-2xl p-5 shadow-sm">
          <h2 className="text-slate-900 font-semibold mb-1">封面缓存</h2>
          <p className="text-slate-400 text-xs mb-4">海报缩略图缓存目录。</p>
          <p className="text-slate-700 text-sm font-mono px-4 py-2.5 rounded-xl bg-slate-50">
            ~/.mona/posters
          </p>
          <p className="text-slate-400 text-xs mt-2">
            注意: 封面缓存目录目前由后端固定,后续如需可配置,可在 config.toml 中扩展字段。
          </p>
        </section>

        {message && (
          <div
            className={`text-sm rounded-xl px-4 py-3 border ${
              message.type === "ok"
                ? "text-green-600 bg-green-50 border-green-200"
                : "text-red-500 bg-red-50 border-red-200"
            }`}
          >
            {message.text}
          </div>
        )}

        <button
          onClick={onSave}
          disabled={saving}
          className="w-full py-3 rounded-xl bg-orange-500 hover:bg-orange-600 disabled:opacity-50 text-white font-semibold transition-colors"
        >
          {saving ? "保存中…" : "保存设置"}
        </button>
      </div>
    </div>
  );
}
