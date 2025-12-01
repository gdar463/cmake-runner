use eyre::Result;
use ratatui::text::Text;
use tokio::sync::mpsc;

pub struct AppIo {
    pub in_tx: mpsc::Sender<Result<String>>,
    pub in_rx: mpsc::Receiver<Result<String>>,
    pub out_tx: mpsc::Sender<Result<Text<'static>>>,
    pub out_rx: mpsc::Receiver<Result<Text<'static>>>,
}

impl Default for AppIo {
    fn default() -> Self {
        let (in_tx, in_rx) = mpsc::channel(50);
        let (out_tx, out_rx) = mpsc::channel(50);
        Self {
            in_tx,
            in_rx,
            out_tx,
            out_rx,
        }
    }
}
