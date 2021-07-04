use std::{
    fs::File,
    io::{BufReader, Read, Write},
    path::{Path, PathBuf},
};

use eyre::Result;
use ignore::WalkBuilder;
use itertools::Itertools;
use log::debug;

use crate::{CliArgs, Format};

#[derive(Debug)]
enum WriteMode {
    Write,
    StdOut,
    DryRun,
}

#[derive(Debug)]
pub(crate) struct App {
    files: Vec<PathBuf>,
    write_mode: WriteMode,
    output: String,
    format: Format,
}

impl App {
    pub(crate) fn new(args: CliArgs) -> Self {
        Self {
            files: get_all_files(&args),
            write_mode: get_write_mode(&args),
            output: args.output,
            format: args.format,
        }
    }

    pub(crate) fn run(self) -> Result<()> {
        debug!("running {:#?}", &self);
        let mut output = File::create(&self.output)?;

        for yaml_value in self
            .files
            .iter()
            .map(|path| self.convert_to_yaml(path))
            .flatten()
            .flatten()
        {
            let yaml_bytes = serde_yaml::to_vec(&yaml_value)?;
            output.write_all(&yaml_bytes)?;
        }

        output.flush()?;

        Ok(())
    }

    fn convert_to_yaml(&self, path: &Path) -> Result<Vec<serde_yaml::Value>> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut yamls = Vec::new();

        match path.extension().and_then(|path| path.to_str()) {
            Some("yaml") => {
                let mut yaml_string = String::new();
                reader.read_to_string(&mut yaml_string)?;

                for yaml_str in yaml_string.split("---") {
                    yamls.push(serde_yaml::from_str(yaml_str)?)
                }
            }
            _ => yamls.push(serde_json::from_reader(reader)?),
        };

        Ok(yamls)
    }
}

fn get_write_mode(args: &CliArgs) -> WriteMode {
    match args {
        CliArgs { dry_run: true, .. } => WriteMode::DryRun,
        CliArgs { write: true, .. } => WriteMode::Write,
        CliArgs { std_out: true, .. } => WriteMode::StdOut,
        _ => WriteMode::DryRun,
    }
}

fn get_all_files(args: &CliArgs) -> Vec<PathBuf> {
    let files = &args.files;

    files
        .iter()
        .map(|path| Path::new(path).to_owned())
        .flat_map(|starting_path| {
            WalkBuilder::new(starting_path)
                .build()
                .filter_map(Result::ok)
                .filter(|f| f.path().is_file())
                .filter(|f| {
                    matches!(
                        f.path().extension().and_then(|path| path.to_str()),
                        Some("yaml" | "json")
                    )
                })
                .map(|file| file.path().to_owned())
        })
        .unique()
        .collect()
}
