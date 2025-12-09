use std::path::Path;

use cmake_runner::app::App;
use eyre::Result;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let dir = std::env::args().nth(1).unwrap_or(".".to_string());
    let path = Path::new(&dir)
        .canonicalize()
        .ok()
        .map(|p| p.join("CMakeLists.txt"))
        .filter(|p| p.exists())
        .ok_or_else(|| eyre::eyre!("Enter a valid directory containing a CMakeLists.txt file."))?;

    let mut terminal = ratatui::init();
    terminal.clear()?;
    let result = App::new(path).run(&mut terminal).await;
    ratatui::restore();
    result
}
