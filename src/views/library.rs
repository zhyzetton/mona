use gpui_kit::{div, Context, IntoElement, ParentElement, Render, Styled, Window};

pub struct LibraryPage;

impl Render for LibraryPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .p_4()
            .child("LibraryPage")
    }
}
