mod app;
pub mod cli;

use app::{App, Format};
use clap::Parser;
use eyre::Result;
use log::{error, info};
use std::fmt::Display;

/// Combine JSON/YAML files into a single file
#[derive(Debug, Parser)]
#[command(author, version, about)]
#[command(arg_required_else_help(true))]
#[command(styles=cli::get_styles())]
pub(crate) struct CliArgs {
    #[arg(
        name = "FILES_OR_FOLDERS",
        help = "File(s) or folder you want to run in",
        required_unless_present = "std_in",
        conflicts_with = "std_in"
    )]
    pub(crate) files: Option<Vec<String>>,

    #[arg(
        long = "stdin",
        short = 'i',
        help = "Read from STDIN",
        required_unless_present = "FILES_OR_FOLDERS",
        conflicts_with = "FILES_OR_FOLDERS"
    )]
    pub(crate) std_in: bool,

    #[arg(
        long,
        short,
        default_value = "1",
        help = "Number of folder depth to recurse into"
    )]
    pub(crate) depth: usize,

    #[arg(
        default_value = "combined.yaml",
        short,
        long,
        help = "Output file name"
    )]
    pub(crate) output: String,

    #[arg(long, help = "Default mode")]
    pub(crate) dry_run: bool,

    #[arg(long, short, help = "Write new output file")]
    pub(crate) write: bool,

    #[arg(
        long = "stdout",
        short,
        help = "Outputs combined file contents to STDOUT"
    )]
    pub(crate) std_out: bool,

    #[arg(
        long,
        short,
        default_value = "yaml",
        help = "The format for the output file, defaults to yaml"
    )]
    pub(crate) format: Format,
}

impl Default for Format {
    fn default() -> Self {
        Self::Yaml
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Yaml => write!(f, "yaml"),
            Self::Json => write!(f, "json"),
            Self::JsonArray => write!(f, "json-array"),
            Self::JsonK8s => write!(f, "json-k8s"),
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();
    color_eyre::install()?;

    let args = CliArgs::parse();

    let app = App::new(args);

    match app.run() {
        Ok(_) => info!("Ran successfully"),
        Err(error) => error!("Unable to combine files: {:?}", error),
    }

    Ok(())
}
