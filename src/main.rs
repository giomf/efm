mod candidate;
mod cli;
mod config;
mod ui;
mod update;

use anyhow::{Context, Result};
use candidate::get_candidates;
use clap::Parser;
use cli::Cli;
use config::{Config, MemberInfo};
use std::path::Path;

const CONFIG_DEFAULT_PATH: &str = "./config.toml";

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config_path = cli.config.unwrap_or(CONFIG_DEFAULT_PATH.to_string());
    let config_path = Path::new(&config_path);
    let mut config = Config::load(config_path).context("Failed to load config")?;

    match cli.command {
        cli::Commands::Adopt => {
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
            let member = ui::prompt_select("Select a member to update:", config.members)?;
            update::update_firmware(&arguments.firmware, &member.hostname).with_context(|| {
                format!(
                    "Error occurred while uploading file: {}",
                    arguments.firmware
                )
            })?;
        }
    };

    Ok(())
}
