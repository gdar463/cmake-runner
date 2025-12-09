use std::process::Stdio;

use ansi_to_tui::IntoText;
use eyre::Result;
use ratatui::{style::Stylize, text::Text};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    process::Command,
    sync::mpsc,
};

pub async fn spawn_command(
    out: &mpsc::Sender<Result<Text<'static>>>,
    mut in_rx: Option<mpsc::Receiver<Result<String>>>,
    command: &str,
    args: &[&str],
    path: &str,
    action: &str,
) -> Result<()> {
    let mut child = Command::new(command)
        .args(args)
        .current_dir(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()?;

    if let Some(mut in_rx) = in_rx.take() {
        let mut stdin = child.stdin.take().expect("stdin not piped");
        tokio::spawn(async move {
            while let Some(Ok(input)) = in_rx.recv().await {
                if stdin.write_all(input.as_bytes()).await.is_err() {
                    break;
                }
            }
        });
    }

    let stdout = child.stdout.take().expect("stdout not piped");
    let stderr = child.stderr.take().expect("stderr not piped");

    let mut stdout_reader = BufReader::new(stdout);
    let mut stderr_reader = BufReader::new(stderr);

    let out_clone = out.clone();
    tokio::spawn(async move {
        let mut buffer = [0; 1024];
        loop {
            match stdout_reader.read(&mut buffer).await {
                Ok(0) => break,
                Ok(n) => {
                    let text = (&buffer[..n]).into_text().unwrap();
                    if out_clone.send(Ok(text)).await.is_err() {
                        break;
                    }
                }
                Err(e) => {
                    if out_clone.send(Err(e.into())).await.is_err() {
                        break;
                    }
                }
            }
        }
    });

    let out_clone = out.clone();
    tokio::spawn(async move {
        let mut buffer = [0; 1024];
        loop {
            match stderr_reader.read(&mut buffer).await {
                Ok(0) => break,
                Ok(n) => {
                    let text = (&buffer[..n]).into_text().unwrap().light_magenta();
                    if out_clone.send(Ok(text)).await.is_err() {
                        break;
                    }
                }
                Err(e) => {
                    if out_clone.send(Err(e.into())).await.is_err() {
                        break;
                    }
                }
            }
        }
    });

    let status = child.wait().await?;
    out.send(Ok(Text::raw("\n"))).await?;
    if status.success() {
        out.send(Ok(Text::raw(format!("=== {action} finished")).light_green()))
            .await?;
        out.send(Ok(Text::raw("\n"))).await?;

        Ok(())
    } else {
        out.send(Ok(Text::raw(format!("=== {action} failed")).light_red()))
            .await?;
        out.send(Ok(Text::raw("\n"))).await?;
        Err(eyre::eyre!(""))
    }
}
