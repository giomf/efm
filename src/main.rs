mod candidate;
mod cli;
mod config;
mod member;
mod ui;

use anyhow::{Context, Result};
use candidate::get_candidates;
use clap::Parser;
use cli::{Cli, Commands, UpdateArguments};
use colored::Colorize;
use config::Config;
use member::Member;
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::Path,
};

const CONFIG_DEFAULT_PATH: &str = "./config.toml";
const IMAGE_VERSION_OFFSET: u64 = 48;
const IMAGE_VERSION_LENGTH: u64 = 32;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config_path = cli.config.unwrap_or(CONFIG_DEFAULT_PATH.to_string());
    let config_path = Path::new(&config_path);
    let mut config = Config::load(config_path).context("Failed to load config")?;

    match cli.command {
        Commands::Adopt => adopt(&mut config, config_path)?,
        Commands::Update(arguments) => update(config, arguments)?,
    };

    Ok(())
}

fn adopt(config: &mut Config, config_path: &Path) -> Result<()> {
    let candidates = get_candidates()?;
    let mut members: Vec<Member> = ui::prompt_multiselect(
        &format!(
            "Found {} candidate(s) to adopt. Please select:",
            candidates.len().to_string().yellow()
        ),
        candidates,
    )?
    .into_iter()
    .map(Member::from)
    .collect();

    config.members.append(&mut members);
    config.save(config_path).context("Failed to save config")?;

    Ok(())
}

fn update(config: Config, arguments: UpdateArguments) -> Result<()> {
    let member = ui::prompt_select("Select a member to update:", config.members)?;
    let member_version = member.status()?.version;
    let update_version = extract_value_from_image(
        &arguments.firmware,
        IMAGE_VERSION_OFFSET,
        IMAGE_VERSION_LENGTH,
    )?;

    let result = ui::promt_confirm(&format!(
        "Current version is {}. Update to {}?",
        member_version.yellow(),
        update_version.green()
    ))?;

    if result {
        member
            .update(&arguments.firmware)
            .context("Failed updating firmware")?;
        println!("Firmware update successful");
    }

    Ok(())
}

fn extract_value_from_image(file: &str, offset: u64, length: u64) -> Result<String> {
    let mut file = File::open(file)?;
    file.seek(SeekFrom::Start(offset))?;

    let mut buffer = vec![0; length as usize];
    let bytes_read = file.read(&mut buffer)?;

    // Truncate buffer if fewer bytes were read
    buffer.truncate(bytes_read);
    // Convert the buffer to a String
    let version = String::from_utf8(buffer)?;
    Ok(version)
}
