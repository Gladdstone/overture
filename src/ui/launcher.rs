use gpui::{
    actions,
    AsyncApp,
    Context,
    Entity,
    prelude::*,
    rgba,
    Window,
    WindowOptions,
};
use gpui_component::{
    list::ListState,
    v_flex
};

use crate::ui::{ ListDisplay, SearchBar };
use crate::window::init_bounds;


actions!([Cancel, Confirm]);

pub struct Launcher {
    listdisplay: Entity<ListState<ListDisplay>>,
    searchbar: Entity<SearchBar>,
}

pub fn init(cx: &mut AsyncApp) -> Option<WindowOptions> {
    // cx.bind_keys([
    //     KeyBinding::new("up", SelectPrev, Some("LauncherView")),
    //     KeyBinding::new("down", SelectNext, Some("LauncherView")),
    //     KeyBinding::new("enter", Confirm, Some("LauncherView")),
    //     KeyBinding::new("escape", Cancel, Some("LauncherView")),
    // ]);

    init_bounds(cx)
}

impl Launcher {
    pub fn new(listdisplay: Entity<ListState<ListDisplay>>, searchbar: Entity<SearchBar>) -> Self {
        Self {
            listdisplay,
            searchbar
        }
    }

    fn cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
       println!("hide window"); 
    }

    fn confirm(&mut self, _: &Confirm, _window: &mut Window, cx: &mut Context<Self>) {
        println!("");
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

