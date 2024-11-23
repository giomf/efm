use anyhow::Result;
use inquire::{
    ui::{RenderConfig, StyleSheet},
    Confirm, MultiSelect, Select,
};
use std::{fmt::Display, sync::LazyLock};

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
