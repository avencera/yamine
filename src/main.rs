mod app;

use app::App;
use eyre::Result;
use log::{error, info};
use std::{fmt::Display, str::FromStr};
use structopt::{clap::AppSettings, StructOpt};

#[derive(Debug)]
pub enum Format {
    Yaml,
    JsonArray,
    JsonK8s,
}

impl Default for Format {
    fn default() -> Self {
        Self::Yaml
    }
}

impl FromStr for Format {
    type Err = String;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.to_lowercase().as_ref() {
            "yaml" => Ok(Self::Yaml),
            "json-array" => Ok(Self::JsonArray),
            "k8s-json" => Ok(Self::JsonK8s),
            "kubernetes-json" => Ok(Self::JsonK8s),
            "json-k8s" => Ok(Self::JsonK8s),
            "json-kubernetes" => Ok(Self::JsonK8s),
            _ => Ok(Self::Yaml),
        }
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

/// Combine JSON/YAML files into a single YAML file
#[derive(Debug, StructOpt)]
#[structopt(name("yamine"), global_settings = &[AppSettings::ColoredHelp, AppSettings::ArgRequiredElseHelp])]
pub(crate) struct CliArgs {
    #[structopt(
        name = "FILES_OR_FOLDERS",
        help = "file(s) or folder you want to run in",
        min_values = 1,
        required = true
    )]
    pub(crate) files: Vec<String>,

    #[structopt(
        long,
        short,
        default_value = "1",
        help = "number of folder depth to recurse into"
    )]
    pub(crate) depth: usize,

    #[structopt(
        default_value = "combined.yaml",
        short,
        long,
        help = "output file name"
    )]
    pub(crate) output: String,

    #[structopt(long, help = "default mode")]
    pub(crate) dry_run: bool,

    #[structopt(long, short, help = "write new output file")]
    pub(crate) write: bool,

    #[structopt(long, short, help = "outputs combined file contents to STDOUT")]
    pub(crate) std_out: bool,

    #[structopt(
        long,
        short,
        default_value,
        help = "the format for the output file, defaults to yaml, options are: 'yaml', 'json', 'k8s-json'"
    )]
    pub(crate) format: Format,
}

fn main() -> Result<()> {
    env_logger::init();
    color_eyre::install()?;

    let args = CliArgs::from_args();

    let app = App::new(args);

    match app.run() {
        Ok(_) => info!("Ran successfully"),
        Err(error) => error!("Unable to combine files: {:?}", error),
    }

    Ok(())
}
