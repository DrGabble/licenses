mod dependency;
mod file_name;
mod local;
mod package;
mod remote;
mod warning;

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();
    let deps =
        dependency::dependencies(&args.working_directory, &args.excluded, &args.search_remote)?;
    warning::print_warnings(&deps);
    Ok(())
}

#[derive(Parser)]
struct Arguments {
    #[clap(short, long)]
    excluded: Vec<String>,
    #[clap(short, long)]
    search_remote: SearchRemote,
    working_directory: PathBuf,
}

#[derive(ValueEnum, Clone, Default)]
enum SearchRemote {
    Never,
    #[default]
    Auto,
    Always,
}
