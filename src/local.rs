use crate::file_name;
use std::path::{Path, PathBuf};

pub fn license_file_paths(folder: &Path) -> impl Iterator<Item = PathBuf> {
    std::fs::read_dir(folder)
        .expect("failed to read directory")
        .filter_map(|entry| match entry {
            Ok(entry) if is_license(&entry.path()) => Some(entry.path()),
            _ => None,
        })
}

fn is_license(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(file_name::is_license)
        .unwrap_or(false)
}
