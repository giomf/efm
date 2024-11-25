mod candidate;
mod cli;
mod config;
mod member;
mod ui;

use anyhow::{Context, Result};
use candidate::get_candidates;
use clap::Parser;
use cli::{Cli, Commands, StatusArguments, UpdateArguments};
use colored::Colorize;
use config::Config;
use member::{Member, MemberStatus};
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::Path,
};
use ui::TICK;

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
        Commands::Status(arguments) => status(&config, arguments)?,
        Commands::List => list(&config),
    };

    Ok(())
}

fn list(config: &Config) {
    let members: Vec<_> = config
        .members
        .iter()
        .map(|member| vec![member.hostname.clone()])
        .collect();
    let table = ui::table(vec!["Host"], members);
    println!("{table}");
}

fn status(config: &Config, arguments: StatusArguments) -> Result<()> {
    let spinner = ui::spinner_start("Fetch member status");

    let status = config
        .members
        .iter()
        .filter(|member| {
            arguments
                .hostname
                .as_ref()
                .map_or(true, |hostname| member.hostname == *hostname)
        })
        .map(|member| match member.status() {
            Ok(information) => MemberStatus::Online(information),
            Err(_) => MemberStatus::Offline(member.hostname.clone()),
        })
        .map(|status| match status {
            MemberStatus::Online(info) => {
                vec![info.hostname, "Online".green().to_string(), info.version]
            }
            MemberStatus::Offline(hostname) => {
                vec![
                    hostname,
                    "Offline".red().to_string(),
                    "n/a".red().to_string(),
                ]
            }
        })
        .collect();

    spinner.finish();

    let table = ui::table(vec!["Host", "State", "Version"], status);
    println!("{table}");
    Ok(())
}

fn adopt(config: &mut Config, config_path: &Path) -> Result<()> {
    let spinner = ui::spinner_start("Scan network for candidates");
    let candidates = get_candidates(&config.members)?;
    spinner.finish();

    if candidates.is_empty() {
        println!("No candidates found to adopt");
        return Ok(());
    }

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
    let spinner = ui::spinner_start("Fetching member status");
    let member_version = member.status()?.version;
    let update_version = extract_value_from_image(
        &arguments.firmware,
        IMAGE_VERSION_OFFSET,
        IMAGE_VERSION_LENGTH,
    )?;
    spinner.finish();

    let result = ui::promt_confirm(&format!(
        "Current version is {}. Update to {}?",
        member_version.yellow(),
        update_version.green()
    ))?;

    if result {
        let file = File::open(&arguments.firmware)
            .with_context(|| format!("Failed to open firmware {}", arguments.firmware))?;
        let file_size = file.metadata()?.len();
        let progress_bar = ui::progressbar(file_size);
        let reader = progress_bar.wrap_read(file);

        member
            .update(reader, file_size)
            .context("Failed updating firmware")?;
        progress_bar.finish_and_clear();
        println!("{} Firmware update successful", TICK.green());
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
