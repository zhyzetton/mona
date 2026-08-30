// 与 Rust 端 media::model::Media 对应的类型
export interface Media {
  id: number | null;
  title: string;
  year: string | null;
  overview: string | null;
  // Rust 端 MediaType 枚举序列化为字符串: "Movie" / "Series" / "Anime" / "Local"
  media_type: string;
  duration: string | null;
  rating: number | null;
  actors: string[];
  poster_path: string | null;
  // 详情页大图(海报大图/截图)
  detail_img_path: string | null;
  file_path: string;
  // 文件大小,如 "1.2 GB"
  file_size: string;
  // 分辨率(高度),如 1080
  resolution: number;
}

// 与 Rust 端 config::Config 对应的类型
export interface Config {
  local_dirs: string[];
  player_name: string | null;
}
