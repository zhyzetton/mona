use std::cmp::PartialEq;
use gpui_kit::{Context, Entity, IntoElement, ParentElement, Render, Window};
use gpui_kit::assets::IconName;
use gpui_kit::component::sidebar::{Sidebar, SidebarFooter, SidebarHeader, SidebarMenu, SidebarMenuItem};

pub struct SideMenu {
    pub state: Entity<SideMenuState>
}

pub struct SideMenuState {
    pub current_menu: MenuItem
}

#[derive(Clone, Copy, PartialEq)]
pub enum MenuItem {
    Home,
    Library,
    Setting
}

impl SideMenu {
    pub fn new(state: Entity<SideMenuState>, cx: &mut Context<Self>) -> Self {
        Self { state }
    }
}


impl Render for SideMenu {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let current_menu = self.state.read(cx).current_menu;

        let menu_list = [
            (MenuItem::Home, "Home", IconName::House),
            (MenuItem::Library, "Library", IconName::Library),
            (MenuItem::Setting, "Setting", IconName::Settings),
        ];

        let mut sidebar_menu = SidebarMenu::new();

        for (item, label, icon) in menu_list {
            let state_handle = self.state.clone();
            sidebar_menu = sidebar_menu.child(
                SidebarMenuItem::new(label)
                    .icon(icon)
                    .active(current_menu == item)
                    .on_click(move |_, _, cx| {
                        state_handle.update(cx, |state, cx| {
                            state.current_menu = item;
                            cx.notify();
                        })
                    })
            );
        }

        Sidebar::new("sidebar")
            .header(SidebarHeader::new().child("Mona"))
            .child(sidebar_menu)
            .footer(SidebarFooter::new().child("User Profile"))
    }
}