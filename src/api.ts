import { invoke } from "@tauri-apps/api/core";
import { convertFileSrc } from "@tauri-apps/api/core";
import type { Config, Media } from "./types";

export const api = {
  async getVideos(): Promise<Media[]> {
    return await invoke<Media[]>("get_videos");
  },
  async scanVideos(): Promise<number> {
    return await invoke<number>("scan_videos");
  },
  async getConfig(): Promise<Config> {
    return await invoke<Config>("get_config");
  },
  async saveConfig(config: Config): Promise<void> {
    return await invoke("save_config", { config });
  },
  async playVideo(videoId: number): Promise<void> {
    return await invoke("play_video", { videoId });
  },
  async playVideoWithPlayer(videoId: number, player: string): Promise<void> {
    return await invoke("play_video_with_player", { videoId, player });
  },
};

// 把本地文件路径转成可在 webview 中加载的资源 URL
export function assetURL(path: string | null): string | undefined {
  if (!path) return undefined;
  return convertFileSrc(path);
}

// WebView2 (Chromium) 能直接播放的扩展名
const WEBVIEW_PLAYABLE_EXTS = ["mp4", "m4v", "webm", "mov", "mkv"];

// 判断某个视频文件能否在应用内直接用 <video> 播放
// 不能播放的(如 avi/wmv/flv/ts)需要回退到系统播放器
export function canPlayInWebview(filePath: string): boolean {
  const ext = filePath.split(".").pop()?.toLowerCase() ?? "";
  return WEBVIEW_PLAYABLE_EXTS.includes(ext);
}
