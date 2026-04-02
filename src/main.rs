mod dependency;
mod file_name;
mod local;
mod package;
mod remote;

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();
    let deps =
        dependency::dependencies(&args.working_directory, &args.excluded, &args.search_remote)?;
    println!(
        "{} licenses found in {} dependencies",
        deps.iter()
            .map(|d| d.local_licenses.len() + d.remote_licenses.len())
            .sum::<usize>(),
        deps.len()
    );
    for no_license in deps
        .iter()
        .filter(|d| d.local_licenses.is_empty() && d.remote_licenses.is_empty())
    {
        println!("no licenses found for {}", no_license.name);
    }
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
