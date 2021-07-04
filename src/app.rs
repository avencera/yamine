use std::path::{Path, PathBuf};

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
struct App {
    files: Vec<PathBuf>,
    write_mode: WriteMode,
    output: String,
    format: Format,
}

impl App {
    fn new(args: CliArgs) -> Self {
        Self {
            files: get_all_files(&args),
            write_mode: get_write_mode(&args),
            output: args.output,
            format: args.format,
        }
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
                .filter(|f| f.path().is_file())
                .map(|file| file.path().to_owned())
        })
        .unique()
        .collect()
}
