use std::fmt::Display;

use anyhow::Result;
use inquire::{
    ui::{RenderConfig, StyleSheet},
    Confirm, MultiSelect, Select,
};

pub fn prompt_multiselect<T: Display>(message: &str, options: Vec<T>) -> Result<Vec<T>> {
    let render_config = RenderConfig::default_colored()
        .with_help_message(StyleSheet::new().with_fg(inquire::ui::Color::Grey))
        .with_selected_option(Some(
            StyleSheet::new().with_fg(inquire::ui::Color::DarkYellow),
        ));

    let answer = MultiSelect::new(message, options)
        .with_render_config(render_config)
        .prompt()?;

    Ok(answer)
}

pub fn prompt_select<T: Display>(message: &str, options: Vec<T>) -> Result<T> {
    let render_config = RenderConfig::default_colored()
        .with_help_message(StyleSheet::new().with_fg(inquire::ui::Color::Grey))
        .with_selected_option(Some(
            StyleSheet::new().with_fg(inquire::ui::Color::DarkYellow),
        ));

    let answer = Select::new(message, options)
        .with_render_config(render_config)
        .prompt()?;

    Ok(answer)
}

pub fn promt_confirm(message: &str) -> Result<bool> {
    let render_config = RenderConfig::default_colored()
        .with_help_message(StyleSheet::new().with_fg(inquire::ui::Color::Grey))
        .with_selected_option(Some(
            StyleSheet::new().with_fg(inquire::ui::Color::DarkYellow),
        ));
    let answer = Confirm::new(message)
        .with_default(false)
        .with_render_config(render_config)
        .prompt()?;
    Ok(answer)
}
