use std::rc::Rc;
use std::cell::RefCell;
use gpui::{
    App,
    AppContext,
    AsyncApp,
    Bounds,
    Size,
    WindowBackgroundAppearance,
    WindowBounds,
    WindowDecorations,
    WindowHandle,
    WindowOptions,
    point,
};
use gpui_component::{ list::ListState, Root};
use crate::{AppState, Launcher, ListDisplay, SearchBar};


pub fn init_bounds(cx: &mut AsyncApp) -> Option<WindowOptions> {
    let display = cx.update({ |app|
        app.primary_display()
    }).unwrap();

    let display_size;
    if display.is_some() {
        display_size = display.unwrap().bounds().size;
    } else {
        eprintln!("Unable to detect primary display");
        return None;
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

    return Some(options);
}

pub fn create_window(cx: &mut AsyncApp) -> Result<WindowHandle<Root>, Box<dyn std::error::Error>> {
    let options = init_bounds(cx).unwrap();

    let window_handle = cx.open_window(options, |window, cx| {
        let appstate = Rc::new(RefCell::new(cx.new(|_cx| AppState::default())));
        let searchbar = cx.new(|cx| SearchBar::new(window, cx, Rc::clone(&appstate)));

        let delegate = ListDisplay {
            appstate: appstate,
        };
        let listdisplay = cx.new(|cx| ListState::new(delegate, window, cx));

        let launcher = cx.new(|cx| Launcher::new(listdisplay, searchbar));
        cx.new(|cx| Root::new(launcher, window, cx))
    })?;

    window_handle.update(cx, |_root, window, _cx| {
        window.activate_window();
    })?;

    Ok(window_handle)
}

pub fn close_window(handle: &WindowHandle<Root>, cx: &mut App) {
    let _ = handle.update(cx, |_root, window, _cx| {
        window.remove_window();
    });
}

