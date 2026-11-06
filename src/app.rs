use crate::views::sidebar::{MenuItem, SideMenu, SideMenuState};
use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div,
};
use sqlx::SqlitePool;
use crate::views::home::HomePage;
use crate::views::library::LibraryPage;
use crate::views::setting::SettingPage;

pub struct MonaApp {
    pool_handle: SqlitePool,
    rt_handle: tokio::runtime::Handle,
    side_menu: Entity<SideMenu>,
    menu_state: Entity<SideMenuState>,
    home_page: Entity<HomePage>,
    library_page: Entity<LibraryPage>,
    setting_page: Entity<SettingPage>
}

impl Render for MonaApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let current_menu = self.menu_state.read(cx).current_menu;
        let right_content = match current_menu {
            MenuItem::Home => {div().child(self.home_page.clone()).into_any_element()}
            MenuItem::Library => {div().child(self.library_page.clone()).into_any_element()}
            MenuItem::Setting => {div().child(self.setting_page.clone()).into_any_element()}
        };
        div()
            .flex()
            .flex_row()
            .size_full()
            .child(self.side_menu.clone())
            .child(div().flex_1().p_4().child(right_content))
    }
}
impl MonaApp {
    pub fn new(cx: &mut Context<Self>, pool_handle: SqlitePool, rt_handle: tokio::runtime::Handle) -> Self {
        let state = cx.new(|_| SideMenuState {
            current_menu: MenuItem::Home
        });
        cx.observe(&state, |_, _, cx| {
            cx.notify()
        }).detach();
        Self {
            pool_handle,
            rt_handle,
            side_menu: cx.new(|inner_cx| SideMenu::new(state.clone(), inner_cx)),
            menu_state: state,
            home_page: cx.new(|_| HomePage),
            library_page: cx.new(|_| LibraryPage),
            setting_page: cx.new(|_| SettingPage)
        }
    }
}
