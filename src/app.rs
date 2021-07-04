use std::{
    fs::File,
    io::{BufReader, Read, Write},
    path::{Path, PathBuf},
};

use eyre::Result;
use ignore::WalkBuilder;
use itertools::Itertools;

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
        let mut output = File::create(&self.output)?;

        for yaml_bytes in self
            .files
            .iter()
            .map(|path| self.convert_to_yaml(path))
            .flatten()
            .map(|yaml| serde_yaml::to_vec(&yaml))
            .flatten()
        {
            output.write_all(b"---")?;
            output.write_all(&yaml_bytes)?;
        }

        output.flush()?;

        Ok(())
    }

    fn convert_to_yaml(&self, path: &Path) -> Result<serde_yaml::Value> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let yaml = if path.ends_with(".yaml") {
            serde_yaml::from_reader(reader)?
        } else {
            serde_json::from_reader(reader)?
        };

        Ok(yaml)
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
                .max_depth(Some(args.depth))
                .build()
                .filter_map(Result::ok)
                .filter(|f| f.path().ends_with(".json") || f.path().ends_with(".yaml"))
                .filter(|f| f.path().is_file())
                .map(|file| file.path().to_owned())
        })
        .unique()
        .collect()
}
