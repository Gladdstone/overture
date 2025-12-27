use std::rc::Rc;
use std::cell::RefCell;
use gpui::{
    App,
    AppContext,
    Application,
    Bounds,
    Context,
    Entity,
    prelude::*,
    rgba,
    Size,
    Window,
    WindowBackgroundAppearance,
    WindowBounds,
    WindowDecorations,
    WindowOptions,
    point
};
use gpui_component::{
    list::ListState,
    Root,
    v_flex
};

mod listdisplay;
mod searchbar;

use crate::listdisplay::ListDisplay;
use crate::searchbar::SearchBar;


fn main() {
    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);
        let display = cx.primary_display();

        cx.spawn(async move |cx| {
            let display_size;
            if display.is_some() {
                display_size = display.unwrap().bounds().size;
            } else {
                eprintln!("Unable to detect primary display");
                return;
            }

            let window_size = Size::new(display_size.width * 0.8, display_size.height *0.3);
            let window_origin = point(display_size.width * 0.1, display_size.height * 0.2);
            let fullscreen_bounds = Bounds {
                origin: window_origin,
                size: window_size,
            };
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(fullscreen_bounds)),
                titlebar: None,
                focus: true,
                show: true,
                app_id: Some("Launch".to_string()),
                window_background: WindowBackgroundAppearance::Opaque,
                window_decorations: Some(WindowDecorations::Server),
                ..Default::default()
            };

            let _ = cx.open_window(options, |window, cx| {
                let mut appstate = Rc::new(RefCell::new(cx.new(|_cx| AppState::default())));
                let searchbar = cx.new(|cx| SearchBar::new(window, cx, Rc::clone(&appstate)));

                let delegate = ListDisplay {
                    appstate: appstate,
                    selected_index: None,
                };
                let listdisplay = cx.new(|cx| ListState::new(delegate, window, cx));

                let launcher = cx.new(|cx| Launcher::new(listdisplay, searchbar));
                cx.new(|cx| Root::new(launcher, window, cx))
            });

        })
        .detach();
    });
}

#[derive(Default)]
struct AppState {
    pub application_vec: Vec<String>,
}

struct Launcher {
    listdisplay: Entity<ListState<ListDisplay>>,
    searchbar: Entity<SearchBar>,
}

impl Launcher {
    pub fn new(listdisplay: Entity<ListState<ListDisplay>>, searchbar: Entity<SearchBar>) -> Self {
        Self {
            listdisplay,
            searchbar
        }
    }
}

impl Render for Launcher {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .child(self.searchbar.clone())
            .child(self.listdisplay.clone())
            .bg(rgba(0x1e1e1e66))

    }
}

