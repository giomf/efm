mod cli;
mod node;

use anyhow::{Context, Result};
use clap::Parser;
use cli::Cli;
use node::get_candidates;
use reqwest::blocking::Client;
use std::{fs::File, io::Read};

fn update_firmware(file_path: &str, url: &str) -> Result<()> {
    let mut file = File::open(file_path)
        .with_context(|| format!("Failed to open file at path: {}", file_path))?;

    // Create a byte buffer to hold the file contents
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .with_context(|| format!("Failed to read file: {}", file_path))?;

    // Create a blocking reqwest client
    let client = Client::new();

    // Send the binary file as the body with `application/octet-stream`
    let res = client
        .post(url)
        .header("Content-Type", "application/octet-stream")
        .body(buffer) // The binary data
        .send()
        .with_context(|| format!("Failed to send POST request to: {}", url))?;

    // Handle the response from the server
    if res.status().is_success() {
        println!("File uploaded successfully!");
    } else {
        anyhow::bail!("Upload failed with status: {}", res.status());
    }

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Adopt => {
            let candidates = get_candidates()?;
            dbg!(candidates);
        }
        cli::Commands::Update(arguments) => {
            update_firmware(&arguments.firmware, &arguments.url).with_context(|| {
                format!(
                    "Error occurred while uploading file: {}",
                    arguments.firmware
                )
            })?;
        }
    };

    Ok(())
}
