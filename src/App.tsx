import { useEffect, useRef, useState } from "react";
import type { ComponentType } from "react";
import {
  Clapperboard,
  Film,
  Globe,
  History,
  Home as HomeIcon,
  Moon,
  Search,
  Settings,
  Shapes,
  Sparkles,
  Sun,
  Tv,
  User,
} from "lucide-react";
import Home from "./pages/Home";
import LibraryPage, { isCategory, type Category } from "./pages/Library";
import RecentPage from "./pages/Recent";
import SettingsPage from "./pages/Settings";
import Detail from "./pages/Detail";
import Player from "./pages/Player";
import { TitleBar } from "./components/TitleBar";
import { Toaster } from "./components/Toaster";
import type { Media } from "./types";
import "./App.css";

type Tab = "home" | "recent" | Category | "settings";
type Theme = "dark" | "light";

interface NavItem {
  id: Tab;
  label: string;
  icon: ComponentType<{ size?: number; className?: string; strokeWidth?: number }>;
}

const MAIN_NAV: NavItem[] = [
  { id: "home", label: "首页", icon: HomeIcon },
  { id: "recent", label: "最近观看", icon: History },
  { id: "movie", label: "电影", icon: Film },
  { id: "series", label: "剧集", icon: Tv },
  { id: "anime", label: "动画", icon: Sparkles },
  { id: "variety", label: "综艺", icon: Clapperboard },
  { id: "documentary", label: "纪录片", icon: Globe },
  { id: "personal", label: "个人", icon: User },
  { id: "other", label: "其他", icon: Shapes },
];

const SETTINGS_LABEL = "设置";

function readStoredTheme(): Theme {
  try {
    const stored = localStorage.getItem("mona-theme");
    if (stored === "light" || stored === "dark") return stored;
    // 未设置过:跟随系统偏好
    return window.matchMedia?.("(prefers-color-scheme: light)").matches
      ? "light"
      : "dark";
  } catch {
    return "dark";
  }
}

function App() {
  const [tab, setTab] = useState<Tab>("home");
  const [search, setSearch] = useState("");
  const [selected, setSelected] = useState<Media | null>(null);
  const [player, setPlayer] = useState<Media | null>(null);
  // 详情页退出中:先淡出再卸载,与进入的淡入形成对称过渡
  const [leavingDetail, setLeavingDetail] = useState(false);
  const exitTimerRef = useRef<number | null>(null);
  // 每次切到首页/最近观看时自增,触发对应页面重新拉取
  const [homeRefresh, setHomeRefresh] = useState(0);
  const [recentRefresh, setRecentRefresh] = useState(0);
  const [theme, setTheme] = useState<Theme>(readStoredTheme);

  // 主题:写回 <html data-theme> 供 CSS 变量切换,并持久化到 localStorage
  useEffect(() => {
    document.documentElement.setAttribute("data-theme", theme);
    try {
      localStorage.setItem("mona-theme", theme);
    } catch {
      /* 忽略持久化失败 */
    }
  }, [theme]);

  const toggleTheme = () => setTheme((t) => (t === "dark" ? "light" : "dark"));

  const cancelDetailExit = () => {
    if (exitTimerRef.current !== null) {
      window.clearTimeout(exitTimerRef.current);
      exitTimerRef.current = null;
    }
  };

  // 打开详情:取消可能未完成的退出流程,立即进入
  const openDetail = (media: Media) => {
    cancelDetailExit();
    setLeavingDetail(false);
    setSelected(media);
  };

  // 关闭详情:先标记"退出中"(覆盖层淡出),动画结束后再真正卸载
  const closeDetail = () => {
    if (!selected || leavingDetail) return;
    setLeavingDetail(true);
    exitTimerRef.current = window.setTimeout(() => {
      exitTimerRef.current = null;
      setLeavingDetail(false);
      setSelected(null);
    }, 300);
  };

  // 切 tab / 播放器会直接清掉详情,取消未完成的退出计时
  const handleTab = (next: Tab) => {
    cancelDetailExit();
    setLeavingDetail(false);
    if (next === "home") setHomeRefresh((n) => n + 1);
    if (next === "recent") setRecentRefresh((n) => n + 1);
    setTab(next);
    setSelected(null);
    setPlayer(null);
  };

  // 卸载时清理计时器
  useEffect(
    () => () => {
      if (exitTimerRef.current !== null) window.clearTimeout(exitTimerRef.current);
    },
    [],
  );

  // 页面常驻挂载,各自独立滚动容器,用 hidden 切换可见性,保留各自的滚动位置。
  // 注意:详情页是悬浮覆盖层,不参与这里的隐藏(详见 main 部分);
  // 各分类页共用一个 Library 容器,分类间切换只换数据视图
  const pageHidden = (view: Tab) => (tab !== view ? "hidden" : "");
  const categoryHidden = !isCategory(tab) ? "hidden" : "";

  const navButtonClass = (active: boolean) =>
    `group w-full flex items-center gap-3 px-3 py-2 rounded-xl text-sm transition-all duration-200 ${
      active
        ? "bg-tint/10 text-strong font-medium"
        : "text-secondary hover:text-strong hover:bg-tint/5"
    }`;

  const navIconClass = (active: boolean) =>
    active ? "text-orange-400" : "text-mute group-hover:text-secondary";

  return (
    <div className="h-screen flex flex-col overflow-hidden bg-base text-body">
      {/* 全屏播放页:覆盖整个窗口,底下页面保持挂载,返回时滚动位置不丢 */}
      {player && <Player media={player} onBack={() => setPlayer(null)} />}

      {/* 自绘标题栏 */}
      <TitleBar />

      {/* 全局消息通知 */}
      <Toaster />

      {/* 主体:侧边栏 + 内容区 */}
      <div className="flex-1 flex min-h-0">
        {/* 左侧边栏 */}
        <aside className="w-64 shrink-0 border-r border-tint/10 bg-base flex flex-col">
          {/* 品牌区 */}
          <div className="px-5 pt-5 pb-4 flex items-center gap-2.5">
            <div className="w-9 h-9 rounded-xl bg-gradient-to-br from-orange-400 to-orange-600 flex items-center justify-center shadow-lg shadow-orange-500/25">
              <Film size={18} className="text-white" />
            </div>
            <span className="text-lg font-bold text-strong tracking-tight">Mona</span>
          </div>

          {/* 搜索 */}
          <div className="px-4 pb-4">
            <div className="flex items-center gap-2.5 px-3.5 h-9 rounded-xl bg-tint/5 ring-1 ring-tint/10 focus-within:ring-orange-500/40 transition-all">
              <Search size={15} className="text-mute shrink-0" />
              <input
                value={search}
                onChange={(e) => setSearch(e.target.value)}
                placeholder="搜索标题、文件名…"
                className="bg-transparent outline-none text-sm text-body placeholder:text-mute w-full"
              />
            </div>
          </div>

          {/* 主导航 */}
          <nav className="flex-1 px-3 overflow-y-auto">
            <ul className="space-y-0.5">
              {MAIN_NAV.map((item) => {
                const active = tab === item.id && !selected;
                return (
                  <li key={item.id}>
                    <button onClick={() => handleTab(item.id)} className={navButtonClass(active)}>
                      <span className={navIconClass(active)}>
                        <item.icon size={18} strokeWidth={active ? 2.4 : 2} />
                      </span>
                      {item.label}
                    </button>
                  </li>
                );
              })}
            </ul>
          </nav>

          {/* 底部:设置 + 主题切换 */}
          <div className="px-3 py-3 border-t border-tint/10 space-y-0.5">
            {(() => {
              const active = tab === "settings" && !selected;
              return (
                <div className="flex items-center gap-1">
                  <button
                    onClick={() => handleTab("settings")}
                    className={`${navButtonClass(active)} flex-1`}
                  >
                    <span className={navIconClass(active)}>
                      <Settings size={18} strokeWidth={active ? 2.4 : 2} />
                    </span>
                    {SETTINGS_LABEL}
                  </button>
                  <button
                    onClick={toggleTheme}
                    aria-label={theme === "dark" ? "切换到浅色模式" : "切换到深色模式"}
                    title={theme === "dark" ? "切换到浅色模式" : "切换到深色模式"}
                    className="w-10 h-10 shrink-0 flex items-center justify-center rounded-xl text-secondary hover:text-strong hover:bg-tint/5 transition-colors"
                  >
                    {theme === "dark" ? <Sun size={18} /> : <Moon size={18} />}
                  </button>
                </div>
              );
            })()}
          </div>
        </aside>

        {/* 主内容区:各页面常驻挂载,只切可见性,滚动位置不丢 */}
        <main className="relative flex-1 flex min-w-0">
          <div className={`flex-1 overflow-y-auto ${pageHidden("home")}`}>
            <Home
              search={search}
              onOpen={openDetail}
              refreshKey={homeRefresh}
              onShowAll={() => handleTab("personal")}
            />
          </div>
          <div className={`flex-1 overflow-y-auto ${pageHidden("recent")}`}>
            <RecentPage search={search} onOpen={openDetail} refreshKey={recentRefresh} />
          </div>
          <div className={`flex-1 overflow-y-auto ${categoryHidden}`}>
            <LibraryPage
              category={isCategory(tab) ? tab : "personal"}
              search={search}
              onOpen={openDetail}
            />
          </div>
          <div className={`flex-1 overflow-y-auto ${pageHidden("settings")}`}>
            <SettingsPage />
          </div>

          {/* 详情页:悬浮覆盖在当前页之上,与底层页面交叉淡入淡出。
              进入:整层 animate-fade-in;返回:先加 leaving 类淡出(200ms),
              计时结束后才真正卸载,让两个方向过渡对称 */}
          {selected && (
            <div
              key={selected.id ?? selected.file_path}
              className={`absolute inset-0 z-20 overflow-y-auto bg-base ${
                leavingDetail
                  ? "opacity-0 transition-opacity duration-300 ease-in pointer-events-none"
                  : "animate-fade-in"
              }`}
            >
              <Detail media={selected} onBack={closeDetail} onPlay={setPlayer} />
            </div>
          )}
        </main>
      </div>
    </div>
  );
}

export default App;
