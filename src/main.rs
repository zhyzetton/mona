use std::borrow::Cow;
use gpui_kit::{
    application, init, open_window, point, px, size, AppContext, AssetSource,
    Bounds, SharedString, WindowBounds, WindowOptions,
};
use gpui_kit::assets::{icon_assets, Assets};
use crate::db::Database;

mod views;
mod models;
mod app;
mod db;

icon_assets!(ExtraIcons, [House, Library]);

struct AppAssets;

impl AssetSource for AppAssets {
    fn load(&self, path: &str) -> gpui_kit::Result<Option<Cow<'static, [u8]>>> {
        // 先查额外包；命中就返回
        if let Some(bytes) = ExtraIcons.load(path)? {
            return Ok(Some(bytes));
        }
        // 未命中则回退到组件默认包（Settings / ChevronRight 等都靠这条路）
        Assets.load(path)
    }

    fn list(&self, path: &str) -> gpui_kit::Result<Vec<SharedString>> {
        let mut paths = Assets.list(path)?;
        paths.extend(ExtraIcons.list(path)?);
        paths.sort();
        paths.dedup();
        Ok(paths)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let tokio_runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    let db = tokio_runtime.block_on(async {
        Database::init("mona.db").await
    })?;
    let pool_handle = db.pool.clone();
    let rt_handle = tokio_runtime.handle().clone();

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
            cx.new(|cx| app::MonaApp::new(cx, pool_handle, rt_handle))
        })
            .expect("Failed to open window");
    });
    Ok(())
}