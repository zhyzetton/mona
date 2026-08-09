//! 应用外壳:状态、消息、导航栈、窗口。

use iced::widget::{
    button, column, container, row, scrollable, text, text_input, Space,
};
use iced::{window, Center, Element, Fill, Length, Task};

use crate::nav::{self, Route};
use crate::pages;
use crate::theme;

/// 应用消息。
#[derive(Debug, Clone)]
pub enum Message {
    /// 导航到目标路由。
    Navigate(Route),
    /// 返回上一页。
    Back,
    /// 搜索框输入。
    Search(String),
    /// 切换网格 / 列表视图。
    ToggleViewMode,
}

/// 内容区视图模式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Grid,
    List,
}

/// 媒体源(占位:数据层接入前为演示条目)。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    pub id: nav::SourceId,
    pub name: String,
}

/// 应用状态。
#[derive(Debug, Clone)]
pub struct App {
    /// 导航栈,栈顶为当前页。
    routes: Vec<Route>,
    /// 已配置媒体源。
    sources: Vec<Source>,
    /// 网格 / 列表视图。
    view_mode: ViewMode,
    /// 搜索框内容(过滤逻辑待数据层接入)。
    search: String,
}

impl App {
    /// 初始状态 + 启动任务。
    fn boot() -> (Self, Task<Message>) {
        let app = Self {
            routes: vec![Route::Home],
            // TODO(backend): 媒体源配置持久化后由数据库填充。
            sources: vec![Source {
                id: 1,
                name: "本地媒体".into(),
            }],
            view_mode: ViewMode::Grid,
            search: String::new(),
        };
        (app, Task::none())
    }

    /// 当前路由(栈顶)。
    pub fn route(&self) -> &Route {
        self.routes.last().expect("导航栈非空")
    }

    /// 是否可返回。
    pub fn can_back(&self) -> bool {
        self.routes.len() > 1
    }

    /// 已配置媒体源。
    pub fn sources(&self) -> &[Source] {
        &self.sources
    }

    /// 当前视图模式。
    pub fn view_mode(&self) -> ViewMode {
        self.view_mode
    }

    /// 搜索框内容。
    pub fn search(&self) -> &str {
        &self.search
    }

    /// 当前页标题。
    fn title(&self) -> String {
        match self.route() {
            Route::Home => "主页".into(),
            Route::Library(kind) => kind.title().into(),
            Route::Browse(id) => self
                .sources
                .iter()
                .find(|s| s.id == *id)
                .map(|s| s.name.clone())
                .unwrap_or_else(|| "媒体源".into()),
            Route::Detail(_) => "详情".into(),
            Route::Favorites => "收藏".into(),
            Route::Search => "搜索".into(),
            Route::Settings => "设置".into(),
        }
    }

    /// 处理导航:详情页压栈可返回,其余路由重置栈。
    fn navigate(&mut self, route: Route) {
        match route {
            Route::Detail(_) => self.routes.push(route),
            _ => self.routes = vec![route],
        }
    }

    /// 弹栈返回。
    fn back(&mut self) {
        if self.can_back() {
            self.routes.pop();
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(route) => self.navigate(route),
            Message::Back => self.back(),
            Message::Search(query) => self.search = query,
            Message::ToggleViewMode => {
                self.view_mode = match self.view_mode {
                    ViewMode::Grid => ViewMode::List,
                    ViewMode::List => ViewMode::Grid,
                };
            }
        }
        Task::none()
    }

    /// 应用外壳:侧边栏 + 主区(头部 + 滚动内容)。
    fn view(app: &App) -> Element<'_, Message> {
        let content = container(
            scrollable(pages::view(app))
                .width(Fill)
                .height(Fill),
        )
        .width(Fill)
        .height(Fill)
        .padding(20.0);

        row![
            nav::sidebar(app),
            column![header(app), content].width(Fill).height(Fill),
        ]
        .width(Fill)
        .height(Fill)
        .into()
    }
}

/// 主区头部:返回键 + 标题 + 搜索框 + 视图切换。
fn header(app: &App) -> Element<'_, Message> {
    let mut header = row![
        button(container(text("←").size(18).color(theme::TEXT)).padding(6.0))
            .style(theme::icon_button)
            .on_press_maybe(app.can_back().then_some(Message::Back)),
        text(app.title()).size(20).color(theme::TEXT),
        Space::new().width(Fill),
        text_input("搜索媒体…", app.search())
            .width(Length::Fixed(240.0))
            .on_input(Message::Search)
            .on_submit(Message::Navigate(Route::Search)),
    ]
    .spacing(10)
    .align_y(Center);

    if matches!(app.route(), Route::Library(_) | Route::Browse(_)) {
        header = header.push(view_toggle(app.view_mode()));
    }

    container(header)
        .width(Fill)
        .padding([16.0, 0.0])
        .into()
}

/// 网格 / 列表切换按钮组。
fn view_toggle(active: ViewMode) -> Element<'static, Message> {
    let toggle = |label: &'static str, selected: bool| {
        button(
            text(label)
                .size(13)
                .color(if selected { theme::ACCENT } else { theme::TEXT_MUTED }),
        )
        .padding([7.0, 12.0])
        .style(theme::nav_item(selected))
        .on_press(Message::ToggleViewMode)
    };

    row![
        toggle("网格", active == ViewMode::Grid),
        toggle("列表", active == ViewMode::List),
    ]
    .spacing(4)
    .into()
}

/// 启动窗口。
pub fn run() -> iced::Result {
    iced::application(App::boot, App::update, App::view)
        .title("Mona")
        .window(window::Settings {
            size: iced::Size::new(1280.0, 800.0),
            min_size: Some(iced::Size::new(960.0, 600.0)),
            ..Default::default()
        })
        .theme(theme::mona())
        .run()
}
