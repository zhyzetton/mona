// 详情页主色提取:图片缩到 24x24 后按 4bit/通道分桶投票,
// 过滤低饱和与过暗过亮像素(否则提出来的主色是脏灰),
// 输出亮度/饱和度夹到可读区间的 hex;跨域污染或加载失败时返回 null(调用方降级)
const SIZE = 24;
// 会话内缓存:同一张图只提一次
const CACHE = new Map<string, string | null>();

export function getDominantColor(url: string): Promise<string | null> {
  const cached = CACHE.get(url);
  if (cached !== undefined) return Promise.resolve(cached);
  return extract(url).then((color) => {
    CACHE.set(url, color);
    return color;
  });
}

async function extract(url: string): Promise<string | null> {
  try {
    const img = new Image();
    // asset 协议与页面不同源,必须声明 CORS 才能读取像素;被污染时 getImageData 会抛 SecurityError
    img.crossOrigin = "anonymous";
    img.src = url;
    await img.decode();

    const canvas = document.createElement("canvas");
    canvas.width = SIZE;
    canvas.height = SIZE;
    const ctx = canvas.getContext("2d", { willReadFrequently: true });
    if (!ctx) return null;
    ctx.drawImage(img, 0, 0, SIZE, SIZE);
    const { data } = ctx.getImageData(0, 0, SIZE, SIZE);

    const buckets = new Map<number, { count: number; r: number; g: number; b: number }>();
    for (let i = 0; i < data.length; i += 4) {
      const [r, g, b, a] = [data[i], data[i + 1], data[i + 2], data[i + 3]];
      if (a < 125) continue;
      const [, s, l] = rgbToHsl(r, g, b);
      if (s < 0.2 || l < 0.12 || l > 0.92) continue;
      const key = ((r >> 4) << 8) | ((g >> 4) << 4) | (b >> 4);
      const cur = buckets.get(key) ?? { count: 0, r: 0, g: 0, b: 0 };
      cur.count++;
      cur.r += r;
      cur.g += g;
      cur.b += b;
      buckets.set(key, cur);
    }
    if (buckets.size === 0) return null;

    // 票数加权饱和度:大片灰色区域不应盖过小片鲜艳区域
    let bestScore = 0;
    let best: [number, number, number] | null = null;
    for (const v of buckets.values()) {
      const avg: [number, number, number] = [
        v.r / v.count,
        v.g / v.count,
        v.b / v.count,
      ];
      const [, s] = rgbToHsl(...avg);
      const score = v.count * (0.3 + s);
      if (score > bestScore) {
        bestScore = score;
        best = avg;
      }
    }
    if (!best) return null;

    const [h, s, l] = rgbToHsl(...best);
    // 夹到可用区间:偏暗的主色与深色底衔接更自然,同时保证叠白字可读
    const s2 = Math.min(Math.max(s, 0.35), 0.7);
    const l2 = Math.min(Math.max(l, 0.2), 0.38);
    return hslToHex(h, s2, l2);
  } catch (e) {
    // 常见于 asset 协议未带 CORS 头导致 canvas 被污染(SecurityError)
    console.warn("主色提取失败,详情页将不使用主色:", e);
    return null;
  }
}

function rgbToHsl(r: number, g: number, b: number): [number, number, number] {
  const rn = r / 255,
    gn = g / 255,
    bn = b / 255;
  const max = Math.max(rn, gn, bn),
    min = Math.min(rn, gn, bn);
  const l = (max + min) / 2;
  if (max === min) return [0, 0, l];
  const d = max - min;
  const s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
  let h: number;
  if (max === rn) h = ((gn - bn) / d + (gn < bn ? 6 : 0)) / 6;
  else if (max === gn) h = ((bn - rn) / d + 2) / 6;
  else h = ((rn - gn) / d + 4) / 6;
  return [h, s, l];
}

function hslToHex(h: number, s: number, l: number): string {
  const a = s * Math.min(l, 1 - l);
  const f = (n: number) => {
    const k = (n + h * 12) % 12;
    const c = l - a * Math.max(-1, Math.min(k - 3, 9 - k, 1));
    return Math.round(255 * c)
      .toString(16)
      .padStart(2, "0");
  };
  return `#${f(0)}${f(8)}${f(4)}`;
}
