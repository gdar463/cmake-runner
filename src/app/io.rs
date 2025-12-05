use eyre::Result;
use ratatui::text::Text;
use tokio::sync::mpsc;

pub struct AppIo {
    pub out_tx: mpsc::Sender<Result<Text<'static>>>,
    pub out_rx: mpsc::Receiver<Result<Text<'static>>>,
}

impl Default for AppIo {
    fn default() -> Self {
        let (out_tx, out_rx) = mpsc::channel(50);
        Self { out_tx, out_rx }
    }
}
