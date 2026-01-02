use std::future;
use zbus::{interface};
use zbus::connection::Builder;
use tokio::sync::{mpsc, oneshot};


pub enum Command {
    Show,
    RequestHide,
}

pub type IpcResponse = Result<(), String>;

pub enum IpcEvent {
    CommandEvent(Command),

    Show {
        response_tx: oneshot::Sender<IpcResponse>,
    },

    Hide {
        response_tx: oneshot::Sender<IpcResponse>,
    },

    Quit {
        response_tx: oneshot::Sender<IpcResponse>,
    },
}

struct DbusApp {
    tx: mpsc::UnboundedSender<Command>,
}

#[interface(name = "org.example.App")]
impl DbusApp {
    fn show(&self) {
        let _ = self.tx.send(Command::Show);
    }
}

pub async fn run_dbus(tx: mpsc::UnboundedSender<Command>) -> zbus::Result<()> {
    let app = DbusApp { tx };

    let connection = Builder::session()?
        .name("org.example.App")?
        .serve_at("/org/example/App", app)?
        .build()
        .await?;

    future::pending::<()>().await;
    Ok(())
}

