use std::path::Path;

use cmake_runner::app::App;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let path = &std::env::args().nth(1).unwrap_or(".".to_string());
    let Ok(path) = Path::new(path).canonicalize() else {
        eprintln!("Enter a valid directory containing a CMakeLists.txt file.");
        std::process::exit(1);
    };
    let path = path.join("CMakeLists.txt");
    if !path.exists() {
        eprintln!("Enter a valid directory containing a CMakeLists.txt file.");
        std::process::exit(1);
    }
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let result = App::new(path).run(&mut terminal).await;
    ratatui::restore();
    result
}
