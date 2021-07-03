use std::{fmt::Display, str::FromStr};

use eyre::Result;
use structopt::{clap::AppSettings, StructOpt};

#[derive(Debug)]
enum Format {
    Yaml,
    Json,
    K8sJson,
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
            "json" => Ok(Self::Json),
            "k8s-json" => Ok(Self::K8sJson),
            "kubernetes-json" => Ok(Self::K8sJson),
            _ => Ok(Self::Yaml),
        }
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Yaml => write!(f, "yaml"),
            Self::Json => write!(f, "json"),
            Self::K8sJson => write!(f, "k8s-json"),
        }
    }
}

/// Combine JSON/YAML files into a single YAML file
#[derive(Debug, StructOpt)]
#[structopt(name("yamine"), global_settings = &[AppSettings::ColoredHelp, AppSettings::ArgRequiredElseHelp])]
struct CliArgs {
    #[structopt(
        name = "FILES_OR_FOLDERS",
        help = "file(s) or folder you want to run in",
        min_values = 1,
        required = true
    )]
    file: Vec<String>,

    #[structopt(
        default_value = "combined.yaml",
        short,
        long,
        help = "output file name"
    )]
    output: String,

    #[structopt(long, short, help = "default mode")]
    dry_run: bool,

    #[structopt(long, short, help = "write new output file")]
    write: bool,

    #[structopt(long, short, help = "outputs combined file contents to STDOUT")]
    std_out: bool,

    #[structopt(
        long,
        short,
        default_value,
        help = "the format for the output file, defaults to yaml, options are: 'yaml', 'json', 'k8s-json'"
    )]
    format: Format,
}

fn main() -> Result<()> {
    env_logger::init();
    color_eyre::install()?;

    let args = CliArgs::from_args();

    println!("{:#?}", args);
    Ok(())
}
