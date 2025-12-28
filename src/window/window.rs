use gpui::{
    AsyncApp,
    Bounds,
    Size,
    WindowBackgroundAppearance,
    WindowBounds,
    WindowDecorations,
    WindowOptions,
    point,
};


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

