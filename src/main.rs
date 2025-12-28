use std::rc::Rc;
use std::cell::RefCell;
use gpui::{
    App,
    AppContext,
    Application,
};
use gpui_component::{
    list::ListState,
    Root,
};

mod ui;
mod window;

use crate::ui::{ AppState, init, Launcher, ListDisplay, SearchBar };


fn main() {
    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            let options = init(cx).unwrap();

            let _ = cx.open_window(options, |window, cx| {
                let appstate = Rc::new(RefCell::new(cx.new(|_cx| AppState::default())));
                let searchbar = cx.new(|cx| SearchBar::new(window, cx, Rc::clone(&appstate)));

                let delegate = ListDisplay {
                    appstate: appstate,
                };
                let listdisplay = cx.new(|cx| ListState::new(delegate, window, cx));

                let launcher = cx.new(|cx| Launcher::new(listdisplay, searchbar));
                cx.new(|cx| Root::new(launcher, window, cx))
            });

        })
        .detach();
    });
}

