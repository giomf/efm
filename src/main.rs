mod candidate;
mod cli;
mod config;
mod ui;

use anyhow::{Context, Result};
use candidate::get_candidates;
use clap::Parser;
use cli::Cli;
use config::{Config, MemberInfo};
use reqwest::blocking::Client;
use std::{fs::File, io::Read, path::Path};

const CONFIG_DEFAULT_PATH: &str = "./config.toml";

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
    let config_path = cli.config.unwrap_or(CONFIG_DEFAULT_PATH.to_string());
    let config_path = Path::new(&config_path);

    match cli.command {
        cli::Commands::Adopt => {
            let mut config = Config::load(config_path).context("Failed to load config")?;
            let candidates = get_candidates()?;
            let mut members: Vec<MemberInfo> = ui::prompt_multiselect(
                &format!(
                    "Found {} candidate(s) to adopt. Please select:",
                    candidates.len()
                ),
                candidates,
            )?
            .into_iter()
            .map(MemberInfo::from)
            .collect();
            config.members.append(&mut members);
            config.save(config_path).context("Failed to save config")?;
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
