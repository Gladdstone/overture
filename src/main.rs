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

    let tray_tx = tx.clone();

    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        // let _ = rt.block_on(run_dbus(tx)).expect("Failed to establish a connection to dbus");
        rt.block_on(run_dbus(tx)).unwrap();
    });


    std::thread::spawn(move || {
        use ksni::Tray;
        use ksni::menu::*;
        use ksni::TrayMethods;

        struct MyTray {
            tx: mpsc::UnboundedSender<Command>,
        }

        impl Tray for MyTray {
            fn id(&self) -> String {
                env!("CARGO_PKG_NAME").into()
            }

            fn icon_name(&self) -> String {
                "applications-system".into()
            }

            fn title(&self) -> String {
                "My App".into()
            }

            fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
                vec![
                    StandardItem {
                        label: "Show".into(),
                        activate: Box::new(|this: &mut MyTray| {
                            let _ = this.tx.send(Command::Show {});
                        }),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: "Hide".into(),
                        activate: Box::new(|this: &mut MyTray| {
                            let _ = this.tx.send(Command::RequestHide {});
                        }),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: "Quit".into(),
                        activate: Box::new(|_| {
                            std::process::exit(0);
                        }),
                        ..Default::default()
                    }.into(),
                ]
            }
        }

        let tray = MyTray { tx: tray_tx };
        tray.spawn()
    });



    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);

        let mut launcher: Option<WindowHandle<Root>> = None;
        let mut visible = false;

        println!("running...");
        cx.spawn(async move |cx| {
            while let Some(event) = rx.recv().await {
                match event {
                    Command::RequestHide { .. } => {
      
                        if visible {
                            if let Some(window) = launcher.as_ref() {
                                window.update(cx, |_root, _window, cx| {
                                    cx.hide();
                                });
                            } 
                            visible = false;
                        } else {
                            println!("already hidden");
                        }
                    }

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

