use std::{
    fs::File,
    io::{self, BufReader, Read, Write},
    path::{Path, PathBuf},
};

use eyre::Result;
use ignore::WalkBuilder;
use itertools::Itertools;
use log::debug;
use serde_yaml::Value;

use crate::CliArgs;

#[derive(Debug)]
enum WriteMode {
    Write,
    StdOut,
    DryRun,
}

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

        match &self.write_mode {
            WriteMode::Write => {
                let writable_output = File::create(&self.output)?;
                self.write_to_format(writable_output)?;
            }

            WriteMode::StdOut => {
                let stdout = io::stdout();
                let writable_output = stdout.lock();
                self.write_to_format(writable_output)?;
            }

            WriteMode::DryRun => self.dry_run()?,
        };

        Ok(())
    }

    fn dry_run(&self) -> Result<()> {
        use colored::*;

        let files = self
            .files
            .iter()
            .map(|path| format!("  * {}", path.to_string_lossy()))
            .join("\n");

        indoc::printdoc!(
            r###"

            Running in `{}` mode.
            
            The following files will be combined: 

            {}

            The combined file be created at: {}/{}
            
            To create the file run again in write mode using `{}` or `{}`.
            To output the file to STDOUT use `{}` or `{}`
            "###,
            "dry-run".blue(),
            files.white().dimmed(),
            std::env::current_dir()?.to_string_lossy().green(),
            self.output.green(),
            "--write".green(),
            "-w".green(),
            "--std-out".green(),
            "-s".green()
        );

        Ok(())
    }

    fn write_to_format<T: Write>(&self, output: T) -> Result<()> {
        match self.format {
            Format::Yaml => self.write_to_yaml(output)?,
            Format::JsonArray => self.write_to_json_array(output)?,
            Format::JsonK8s => self.write_to_json_kubernetes(output)?,
        }

        Ok(())
    }

    fn write_to_yaml<T: Write>(&self, mut output: T) -> Result<()> {
        for yaml_value in self.get_yamls_iter() {
            let yaml_bytes = serde_yaml::to_vec(&yaml_value)?;
            output.write_all(&yaml_bytes)?;
        }

        output.flush()?;

        Ok(())
    }

    fn write_to_json_array<T: Write>(&self, mut output: T) -> Result<()> {
        output.write_all(b"[")?;

        for (index, yaml_value) in self.get_yamls_iter().enumerate() {
            let json_bytes = serde_json::to_vec(&yaml_value)?;

            if index != 0 {
                output.write_all(b",")?;
            }

            output.write_all(&json_bytes)?;
        }

        output.write_all(b"]")?;
        output.flush()?;

        Ok(())
    }

    fn write_to_json_kubernetes<T: Write>(&self, mut output: T) -> Result<()> {
        output.write_all(br##"{"kind": "List", "apiVersion": "v1", "items": ["##)?;

        for (index, yaml_value) in self.get_yamls_iter().enumerate() {
            let json_bytes = serde_json::to_vec(&yaml_value)?;

            if index != 0 {
                output.write_all(b",")?;
            }

            output.write_all(&json_bytes)?;
        }

        output.write_all(b"]}")?;
        output.flush()?;

        Ok(())
    }

    fn get_yamls_iter(&self) -> impl Iterator<Item = Value> + '_ {
        self.files
            .iter()
            .map(|path| convert_to_yaml(path))
            .flatten()
            .flatten()
    }
}

fn convert_to_yaml(path: &Path) -> Result<Vec<Value>> {
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
