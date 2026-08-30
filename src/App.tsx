import { useState } from "react";
import type { ComponentType } from "react";
import { Home as HomeIcon, Library, Settings, Search } from "lucide-react";
import Home from "./pages/Home";
import LibraryPage from "./pages/Library";
import SettingsPage from "./pages/Settings";
import Detail from "./pages/Detail";
import Player from "./pages/Player";
import type { Media } from "./types";
import "./App.css";

type Tab = "home" | "library" | "settings";

interface NavItem {
  id: Tab;
  label: string;
  icon: ComponentType<{ size?: number; className?: string }>;
}

const NAV: { section: string; items: NavItem[] }[] = [
  {
    section: "",
    items: [
      { id: "home", label: "主屏幕", icon: HomeIcon },
      { id: "library", label: "媒体库", icon: Library },
    ],
  },
  {
    section: "文件",
    items: [{ id: "settings", label: "设置", icon: Settings }],
  },
];

function App() {
  const [tab, setTab] = useState<Tab>("home");
  const [search, setSearch] = useState("");
  const [selected, setSelected] = useState<Media | null>(null);
  const [player, setPlayer] = useState<Media | null>(null);
  // 每次切到主屏幕时自增,触发 Home 重新拉取(扫描后切回来能看到新内容)
  const [homeRefresh, setHomeRefresh] = useState(0);

  const handleTab = (next: Tab) => {
    if (next === "home") setHomeRefresh((n) => n + 1);
    setTab(next);
    setSelected(null);
    setPlayer(null);
  };

  // 页面常驻挂载,各自独立滚动容器,用 hidden 切换可见性,保留各自的滚动位置
  const pageHidden = (view: Tab) => (selected || tab !== view ? "hidden" : "");

  return (
    <div className="min-h-screen bg-[#f2f3f5] text-slate-900 flex">
      {/* 全屏播放页:覆盖整个窗口,底下页面保持挂载,返回时滚动位置不丢 */}
      {player && <Player media={player} onBack={() => setPlayer(null)} />}

      {/* 左侧边栏 */}
      <aside className="w-60 shrink-0 bg-white border-r border-slate-200 flex flex-col">
        <div className="px-4 pt-5 pb-2">
          <div className="flex items-center gap-2 px-3 py-2 rounded-lg bg-slate-100">
            <Search size={16} className="text-slate-400 shrink-0" />
            <input
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              placeholder="搜索"
              className="bg-transparent outline-none text-sm text-slate-700 placeholder-slate-400 w-full"
            />
          </div>
        </div>

        <nav className="flex-1 px-3 py-2 overflow-y-auto">
          {NAV.map((group) => (
            <div key={group.section} className="mb-2">
              {group.section && (
                <p className="px-3 pt-3 pb-1 text-xs font-semibold text-slate-400 uppercase tracking-wide">
                  {group.section}
                </p>
              )}
              <ul>
                {group.items.map((item) => {
                  const active = tab === item.id && !selected;
                  return (
                    <li key={item.id}>
                      <button
                        onClick={() => handleTab(item.id)}
                        className={`w-full flex items-center gap-3 px-3 py-2 rounded-lg text-sm transition-colors ${
                          active
                            ? "bg-orange-50 text-orange-600 font-semibold"
                            : "text-slate-600 hover:bg-slate-50"
                        }`}
                      >
                        <span
                          className={`flex items-center ${active ? "text-orange-500" : "text-slate-400"}`}
                        >
                          <item.icon size={18} />
                        </span>
                        {item.label}
                      </button>
                    </li>
                  );
                })}
              </ul>
            </div>
          ))}
        </nav>
      </aside>

      {/* 主内容区:各页面常驻挂载,只切可见性,滚动位置不丢 */}
      <main className="flex-1 flex h-screen min-w-0">
        <div className={`flex-1 overflow-y-auto ${pageHidden("home")}`}>
          <Home search={search} onOpen={setSelected} refreshKey={homeRefresh} />
        </div>
        <div className={`flex-1 overflow-y-auto ${pageHidden("library")}`}>
          <LibraryPage search={search} onOpen={setSelected} />
        </div>
        <div className={`flex-1 overflow-y-auto ${pageHidden("settings")}`}>
          <SettingsPage />
        </div>
        <div className={`flex-1 overflow-y-auto ${selected ? "" : "hidden"}`}>
          {selected && (
            <Detail
              media={selected}
              onBack={() => setSelected(null)}
              onPlay={setPlayer}
            />
          )}
        </div>
      </main>
    </div>
  );
}

export default App;
