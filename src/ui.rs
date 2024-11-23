use std::fmt::Display;

use anyhow::Result;
use inquire::{
    ui::{RenderConfig, StyleSheet},
    MultiSelect, Select,
};

pub fn prompt_multiselect<T: Display>(prompt: &str, options: Vec<T>) -> Result<Vec<T>> {
    let render_config = RenderConfig::default_colored()
        .with_help_message(StyleSheet::new().with_fg(inquire::ui::Color::Grey))
        .with_selected_option(Some(
            StyleSheet::new().with_fg(inquire::ui::Color::DarkYellow),
        ));

    let answer = MultiSelect::new(prompt, options)
        .with_render_config(render_config)
        .prompt()?;

    Ok(answer)
}

pub fn prompt_select<T: Display>(prompt: &str, options: Vec<T>) -> Result<T> {
    let render_config = RenderConfig::default_colored()
        .with_help_message(StyleSheet::new().with_fg(inquire::ui::Color::Grey))
        .with_selected_option(Some(
            StyleSheet::new().with_fg(inquire::ui::Color::DarkYellow),
        ));

    let answer = Select::new(prompt, options)
        .with_render_config(render_config)
        .prompt()?;

    Ok(answer)
}
