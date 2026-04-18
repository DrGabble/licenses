use crate::license;
use crate::license::License;
use crate::package::{Package, Version};
use std::path::{Path, PathBuf};

pub type Local = License<PathBuf>;

impl Local {
    pub fn location_file_name(&self) -> String {
        self.location
            .file_name()
            .expect("invalid local license file path")
            .to_string_lossy()
            .to_string()
    }
}

pub fn package_local_licenses(keywords: &[String], package: &Package) -> Vec<Local> {
    std::fs::read_dir(&package.project_folder)
        .expect("failed to read directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| is_license(keywords, path))
        .map(|path| Local {
            package: package.name.clone(),
            version: package.version.clone(),
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            location: path,
        })
        .collect()
}

pub fn output_folder_licenses(project_folder: &Path) -> Vec<Local> {
    let entries = match std::fs::read_dir(project_folder) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Vec::new(),
        Err(error) => panic!("failed to read directory: {}", error),
    };
    entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter_map(from_output_folder)
        .collect()
}

fn from_output_folder(location: PathBuf) -> Option<Local> {
    let (package, suffix) = location.file_name()?.to_str()?.split_once('_')?;
    let (version, name) = suffix.split_once('_')?;

    Some(Local {
        package: package.to_string(),
        version: Version::parse(version).ok()?,
        name: name.to_string(),
        location,
    })
}

#[allow(clippy::ptr_arg)]
fn is_license(keywords: &[String], path: &PathBuf) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| license::is_license(keywords, name))
        .unwrap_or(false)
}
