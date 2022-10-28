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

#[derive(Debug, Clone)]
pub enum Format {
    Yaml,
    JsonArray,
    JsonK8s,
}

#[derive(Debug)]
pub(crate) struct App {
    files: Option<Vec<PathBuf>>,
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
            .flatten()
            .map(|path| format!("  * {}", path.to_string_lossy()))
            .join("\n");

        indoc::printdoc!(
            r###"

            Running in `{}` mode.
            
            The following files will be combined: 

            {}

            The combined file be created at: {}/{}
            The combined file will be in `{}` format
            
            To create the file run again in write mode using `{}` or `{}`.
            To output the file to STDOUT use `{}` or `{}`
            "###,
            "dry-run".blue(),
            files.white().dimmed(),
            std::env::current_dir()?.to_string_lossy().green(),
            self.output.green(),
            self.format.to_string().green(),
            "--write".green(),
            "-w".green(),
            "--stdout".green(),
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
            serde_yaml::to_writer(&mut output, &yaml_value)?;
        }

        output.flush()?;

        Ok(())
    }

    fn write_to_json_array<T: Write>(&self, mut output: T) -> Result<()> {
        output.write_all(b"[")?;

        for (index, yaml_value) in self.get_yamls_iter().iter().enumerate() {
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

        for (index, yaml_value) in self.get_yamls_iter().iter().enumerate() {
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

    fn get_yamls_iter(&self) -> Vec<Value> {
        if let Some(files) = &self.files {
            return files
                .iter()
                .flat_map(|path| convert_to_yaml(path))
                .flatten()
                .collect();
        } else {
            let mut yamls = Vec::new();

            // if no files provided, read from stdin
            let mut yaml_string = String::new();
            let mut stdin = std::io::stdin(); // We get `Stdin` here.
            stdin.read_to_string(&mut yaml_string).unwrap();

            for yaml_str in yaml_string.split("---") {
                if let Ok(yaml) = serde_yaml::from_str(yaml_str) {
                    yamls.push(yaml)
                }
            }

            yamls
        }
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

fn get_all_files(args: &CliArgs) -> Option<Vec<PathBuf>> {
    let files = args.files.as_ref()?;

    Some(
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
            .collect(),
    )
}
