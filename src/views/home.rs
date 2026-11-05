use gpui_kit::{div, Context, IntoElement, ParentElement, Render, Styled, Window};

pub struct HomePage;

impl Render for HomePage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .p_4()
            .child("HomePage")
    }
}
