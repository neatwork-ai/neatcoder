use clap::Parser;
use console::{style, Style};
use dialoguer::theme::ColorfulTheme;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

// Commands
// 1. Create an Assistant
// 2. Train your assistant on an API
// 3.

#[derive(Parser)]
pub enum Commands {
    #[clap(action, about = "Creates a new GPT assistant")]
    InitAssistant {
        #[clap(short, long, action, help = "The name of the assistant")]
        name: Option<String>,
    },

    #[clap(
        about = "Link a SuiPlay account to your local configuration. If you already have a SuiPlay account prior to installing the CLI then you can link your account via this command."
    )]
    AddApi {
        #[clap(
            short,
            long,
            help = "This is where all the byte-cli configurations and projects are stored"
        )]
        root_dir: Option<String>,
    },
}

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
