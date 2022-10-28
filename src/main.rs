mod app;

use app::{App, Format};
use clap::Parser;
use eyre::Result;
use log::{error, info};
use std::{fmt::Display, str::FromStr};

/// Combine JSON/YAML files into a single file
#[derive(Debug, Parser)]
#[command(name("yamine"), arg_required_else_help(true))]
pub(crate) struct CliArgs {
    #[arg(
        name = "FILES_OR_FOLDERS",
        help = "File(s) or folder you want to run in",
        min_values = 1,
        required = true
    )]
    pub(crate) files: Vec<String>,

    #[structopt(
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
        help = "The format for the output file, defaults to yaml, options are: 'yaml', 'json-array', 'k8s-json'"
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
            Self::JsonArray => write!(f, "json-array"),
            Self::JsonK8s => write!(f, "json-k8s"),
        }
    }
}

impl FromStr for Format {
    type Err = String;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.to_lowercase().as_ref() {
            "yaml" => Ok(Self::Yaml),
            "json" => Ok(Self::JsonArray),
            "json-array" => Ok(Self::JsonArray),
            "k8s-json" => Ok(Self::JsonK8s),
            "kubernetes-json" => Ok(Self::JsonK8s),
            "json-k8s" => Ok(Self::JsonK8s),
            "json-kubernetes" => Ok(Self::JsonK8s),
            _ => Ok(Self::Yaml),
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
