use gpui::{
    App,
    Application,
    WindowHandle,
};
use gpui_component::{
    list::ListState,
    Root,
};
use tokio::sync::mpsc;

mod core;
mod ui;
mod window;

use crate::ui::{ AppState, Launcher, ListDisplay, SearchBar };
use crate::window::{ Command, IpcEvent, run_dbus };


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = mpsc::unbounded_channel::<Command>();

    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        // let _ = rt.block_on(run_dbus(tx)).expect("Failed to establish a connection to dbus");
        rt.block_on(run_dbus(tx)).unwrap();
    });

    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);

        // let (_tx, mut rx) = mpsc::channel::<IpcEvent>(32);
        let mut launcher: Option<WindowHandle<Root>> = None;
        let mut visible = false;

        println!("running...");
        cx.spawn(async move |cx| {
            while let Some(event) = rx.recv().await {
                println!("listening...");
                println!("{:?}", event);
                match event {
                    // IpcEvent::CommandEvent(Command::RequestHide) if visible => {
                    //     println!("hide");
                    // }

                    Command::Show { .. } => {
                        let result = if !visible {
                            match window::create_window(cx) {
                                Ok(windowhandle) => {
                                    launcher = Some(windowhandle);
                                    visible = true;
                                    Ok(())
                                }
                                Err(e) => {
                                    Err(format!("Failed to create window: {}", e))
                                }
                            }
                        } else {
                            Ok(())
                        };
                    }

                    _ => {}
                }
            }
        })
        .detach();
    });

    Ok(())
}

