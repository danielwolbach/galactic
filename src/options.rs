use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Options {
    #[arg(long, default_value_t = false, help = "Use the default config")]
    pub default_config: bool,

    #[arg(long, help = "Use a custom configuration directory")]
    pub config_path: Option<PathBuf>,
}

impl Options {
    pub fn parse() -> Self {
        let options = Parser::parse();
        tracing::debug!("Parsed options `{options:?}`.");
        options
    }
}
