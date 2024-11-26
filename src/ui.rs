use crate::member::{Member, MemberStatus};
use anyhow::Result;
use colored::Colorize;
use comfy_table::{presets::UTF8_FULL_CONDENSED, ContentArrangement, Table};
use indicatif::{ProgressBar, ProgressStyle};
use inquire::{
    ui::{RenderConfig, StyleSheet},
    Confirm, MultiSelect, Select,
};
use std::{fmt::Display, sync::LazyLock, time::Duration};

const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
const TABLE_HEADER_HOST: &str = "Host";
const TABLE_HEADER_STATE: &str = "State";
const TABLE_HEADER_VERSION: &str = "Version";
const TABLE_HEADER_TIME: &str = "Time";

const SPINNER_TICK: u64 = 100;
pub const TICK: &str = "✔";
const PROGRESS_BAR_TEMPLATE: &str =
    "[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})";

static RENDER_CONFIG: LazyLock<RenderConfig> = LazyLock::new(|| {
    RenderConfig::default_colored()
        .with_help_message(StyleSheet::new().with_fg(inquire::ui::Color::Grey))
        .with_selected_option(Some(
            StyleSheet::new().with_fg(inquire::ui::Color::DarkYellow),
        ))
});

impl Display for Member {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.hostname)
    }
}

pub trait Tableable {
    fn header() -> Vec<String>;
    fn row(&self) -> Vec<String>;
}

impl Tableable for Member {
    fn header() -> Vec<String> {
        vec![
            "Host".to_string(),
            "Version".to_string(),
            "Time".to_string(),
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.hostname.clone(),
            self.version.clone(),
            self.timestamp.format(DATETIME_FORMAT).to_string(),
        ]
    }
}

impl Tableable for MemberStatus {
    fn header() -> Vec<String> {
        vec![
            TABLE_HEADER_HOST.to_string(),
            TABLE_HEADER_STATE.to_string(),
            TABLE_HEADER_VERSION.to_string(),
            TABLE_HEADER_TIME.to_string(),
        ]
    }

    fn row(&self) -> Vec<String> {
        match self {
            MemberStatus::Online(member) => {
                vec![
                    member.hostname.clone(),
                    "Online".to_string(),
                    member.version.clone(),
                    member.timestamp.format(DATETIME_FORMAT).to_string(),
                ]
            }
            MemberStatus::Offline(member) => {
                vec![
                    member.hostname.clone(),
                    "Offline".to_string(),
                    member.version.clone(),
                    member.timestamp.format(DATETIME_FORMAT).to_string(),
                ]
            }
        }
    }
}

pub fn prompt_multiselect<T: Display>(message: &str, options: Vec<T>) -> Result<Vec<T>> {
    let answer = MultiSelect::new(message, options)
        .with_render_config(*RENDER_CONFIG)
        .prompt()?;

    Ok(answer)
}

pub fn prompt_select<T: Display>(message: &str, options: Vec<T>) -> Result<T> {
    let answer = Select::new(message, options)
        .with_render_config(*RENDER_CONFIG)
        .prompt()?;

    Ok(answer)
}

pub fn promt_confirm(message: &str) -> Result<bool> {
    let answer = Confirm::new(message)
        .with_default(false)
        .with_render_config(*RENDER_CONFIG)
        .prompt()?;
    Ok(answer)
}

pub fn spinner_start(message: &str) -> ProgressBar {
    let spinner = ProgressBar::new_spinner()
        .with_style(ProgressStyle::default_spinner().tick_strings(&[
            "⠲",
            "⠴",
            "⠦",
            "⠖",
            TICK.green().to_string().as_str(),
        ]))
        .with_message(message.to_string());
    spinner.enable_steady_tick(Duration::from_millis(SPINNER_TICK));
    spinner
}
pub fn progressbar(length: u64) -> ProgressBar {
    let progress_bar = ProgressBar::new(length);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(PROGRESS_BAR_TEMPLATE)
            .unwrap()
            .progress_chars("#>-"),
    );
    progress_bar
}

pub fn table(header: Vec<String>, rows: Vec<Vec<String>>) -> String {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .set_header(header)
        .set_content_arrangement(ContentArrangement::Dynamic);

    for row in rows {
        table.add_row(row);
    }

    table.to_string()
}
