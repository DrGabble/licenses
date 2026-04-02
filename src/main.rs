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
        dependency::dependencies(&args.project_directory, &args.excluded, &args.search_remote)?;
    warning::print_warnings(&deps);
    std::fs::create_dir_all(&args.output_directory)?;
    for dependency in deps {
        for license_path in dependency.local_licenses {
            let license_name = license_path.file_name().unwrap().to_str().unwrap();
            let file_name = format!("{}-{}", &dependency.name, license_name);
            let output_file = args.output_directory.join(file_name);
            std::fs::copy(license_path, output_file)?;
        }
    }
    Ok(())
}

#[derive(Parser)]
struct Arguments {
    #[clap(short, long)]
    excluded: Vec<String>,
    #[clap(short, long)]
    search_remote: SearchRemote,
    #[clap(short, long, default_value = "./")]
    project_directory: PathBuf,
    #[clap(short, long, default_value = "./licenses/")]
    output_directory: PathBuf,
}

#[derive(ValueEnum, Clone, Default)]
enum SearchRemote {
    Never,
    #[default]
    Auto,
    Always,
}
