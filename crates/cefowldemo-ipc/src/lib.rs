use ipc_channel::ipc::{IpcOneShotServer, IpcReceiver, IpcSender};
use serde::{Deserialize, Serialize};

pub type Receiver = IpcReceiver<Payload>;

#[derive(Debug, Serialize, Deserialize)]
pub struct InitialInfo {
    pub context_id: u32,
    pub window_width: i32,
    pub window_height: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Payload {
    Initialize(InitialInfo),
    SetSize { width: f64, height: f64 },
}

pub struct ParentProcessIpc {
    server: IpcOneShotServer<Payload>,
}

impl ParentProcessIpc {
    pub fn new() -> Result<(Self, String), ipc_channel::Error> {
        let (server, token) = IpcOneShotServer::new()?;

        Ok((Self { server }, token))
    }

    pub fn accept(self) -> anyhow::Result<(Receiver, Payload)> {
        Ok(self.server.accept()?)
    }
}

#[derive(Clone)]
pub struct ChildProcessIpc {
    tx: IpcSender<Payload>,
}

impl ChildProcessIpc {
    pub fn connect(token: String) -> Result<Self, ipc_channel::Error> {
        Ok(Self {
            tx: IpcSender::connect(token)?,
        })
    }

    pub fn send(&self, payload: Payload) -> Result<(), ipc_channel::Error> {
        self.tx.send(payload)?;
        Ok(())
    }
}
