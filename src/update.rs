use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use std::fs::File;

const PROGRESS_BAR_TEMPLATE: &str =
    "[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})";

pub fn update_firmware(file_path: &str, host: &str) -> Result<()> {
    let file =
        File::open(file_path).with_context(|| format!("Failed to open firmware {}", file_path))?;
    let file_size = file.metadata()?.len();

    let progress_bar = ProgressBar::new(file_size);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(PROGRESS_BAR_TEMPLATE)?
            .progress_chars("#>-"),
    );

    let reader = progress_bar.wrap_read(file);

    let url = format!("http://{host}/update");
    let res = Client::new()
        .post(&url)
        .header("Content-Type", "application/octet-stream")
        .body(reqwest::blocking::Body::sized(reader, file_size))
        .send()
        .with_context(|| format!("Failed to update firmware of {}", host))?;

    // Handle the response from the server
    if res.status().is_success() {
        println!("Firmware update successful!");
    } else {
        anyhow::bail!("Failed to update frimware of {} ({})", host, res.status());
    }

    Ok(())
}
