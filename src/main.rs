use std::borrow::Cow;
use gpui_kit::{
    application, init, open_window, point, px, size, AppContext, AssetSource,
    Bounds, Result, SharedString, WindowBounds, WindowOptions,
};
use gpui_kit::assets::{icon_assets, Assets};

mod views;
mod models;
mod app;

icon_assets!(ExtraIcons, [House, Library]);

struct AppAssets;

impl AssetSource for AppAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        // 先查额外包；命中就返回
        if let Some(bytes) = ExtraIcons.load(path)? {
            return Ok(Some(bytes));
        }
        // 未命中则回退到组件默认包（Settings / ChevronRight 等都靠这条路）
        Assets.load(path)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut paths = Assets.list(path)?;
        paths.extend(ExtraIcons.list(path)?);
        paths.sort();
        paths.dedup();
        Ok(paths)
    }
}

fn main() {
    application().with_assets(AppAssets).run(|cx| {
        init(cx);

        let options = WindowOptions {

            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.0), px(100.0)), // 初始弹出位置
                size: size(px(1200.0), px(800.0)),   // 初始宽高
            })),
            ..WindowOptions::default()
        };

        open_window(options, cx, |_, cx| {
            cx.new(|cx| app::MonaApp::new(cx))
        })
            .expect("Failed to open window");
    });
}