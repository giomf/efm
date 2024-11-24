use anyhow::Result;
use colored::Colorize;
use comfy_table::{presets::UTF8_FULL_CONDENSED, ContentArrangement, Table};
use indicatif::{ProgressBar, ProgressStyle};
use inquire::{
    ui::{RenderConfig, StyleSheet},
    Confirm, MultiSelect, Select,
};
use std::{fmt::Display, sync::LazyLock, time::Duration};

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

pub fn table(header: Vec<&str>, rows: Vec<Vec<String>>) -> String {
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
