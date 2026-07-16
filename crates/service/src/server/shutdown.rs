use tokio::{
    select,
    signal::unix::{Signal, SignalKind, signal},
};
use tokio_util::sync::CancellationToken;

use crate::shared::{error::Result, logger::info};

#[derive(Debug)]
pub struct ShutdownSignal {
    pub ctrl_c: Signal,
    pub sigterm: Signal,
}

impl ShutdownSignal {
    pub fn new() -> Result<Self> {
        let ctrl_c = signal(SignalKind::interrupt())?;
        let sigterm = signal(SignalKind::terminate())?;
        Ok(Self { ctrl_c, sigterm })
    }

    pub async fn wait_with_cancel(self, cancellation: CancellationToken) {
        let Self { mut ctrl_c, mut sigterm } = self;

        select! {
            _ = ctrl_c.recv() => {
                info!("Received SIGINT (Ctrl+C) signal");
            },
            _ = sigterm.recv() => {
                info!("Received SIGTERM signal");
            },
            _ = cancellation.cancelled() => {
                info!("Cancellation token cancelled");
            },
        }
    }
}
