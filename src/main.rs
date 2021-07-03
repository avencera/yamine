use eyre::Result;
use structopt::{clap::AppSettings, StructOpt};

/// Combine JSON/YAML files into a single YAML file
#[derive(Debug, StructOpt)]
#[structopt(name("yamine"), global_settings = &[AppSettings::ColoredHelp, AppSettings::ArgRequiredElseHelp])]
struct CliArgs {
    #[structopt(
        name = "FILES_OR_FOLDERS",
        help = "file(s) or folder you want to run in, if run in a folder it will combine all json and yaml files in the folder into one",
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
}

fn main() -> Result<()> {
    env_logger::init();
    color_eyre::install()?;

    let args = CliArgs::from_args();

    println!("{:#?}", args);
    Ok(())
}
