use console::{style, Style};
use dialoguer::{theme::ColorfulTheme, MultiSelect, Select};

pub fn get_dialoguer_theme() -> ColorfulTheme {
    ColorfulTheme {
        prompt_style: Style::new(),
        checked_item_prefix: style("✔".to_string()).green().force_styling(true),
        unchecked_item_prefix: style("✔".to_string())
            .black()
            .force_styling(true),
        ..Default::default()
    }
}

pub fn select<'a>(
    theme: &ColorfulTheme,
    prompt: &str,
    options: &[&'a str],
) -> anyhow::Result<&'a str> {
    let index = Select::with_theme(theme)
        .with_prompt(prompt)
        .items(options)
        .interact()
        .unwrap();

    Ok(options[index])
}

pub fn multi_select(
    theme: &ColorfulTheme,
    prompt: &str,
    option_fields: &Vec<&str>,
) -> anyhow::Result<Vec<String>> {
    let indexes = MultiSelect::with_theme(theme)
        .with_prompt(prompt)
        .items(option_fields)
        .interact()
        .unwrap();

    let borrowed = indexes
        .iter()
        .map(|i| option_fields[*i].to_string())
        .collect::<Vec<_>>();

    Ok(borrowed)
}
