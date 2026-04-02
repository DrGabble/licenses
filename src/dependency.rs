use crate::{SearchRemote, local, package, remote};
use cargo_metadata::semver::Version;
use std::path::{Path, PathBuf};

pub struct Dependency {
    pub name: String,
    pub version: Version,
    pub local_licenses: Vec<PathBuf>,
    pub remote_licenses: Vec<url::Url>,
}

pub fn dependencies(
    working_directory: &Path,
    excluded: &[String],
    search_remote: &SearchRemote,
) -> anyhow::Result<Vec<Dependency>> {
    package::dependencies(working_directory, excluded)?
        .map(|package| -> anyhow::Result<Dependency> {
            let local: Vec<_> = local::license_file_paths(&package.project_folder).collect();
            let remote = remote_licenses(&package.repository, &local, search_remote)?;
            Ok(Dependency {
                name: package.name,
                version: package.version,
                local_licenses: local,
                remote_licenses: remote,
            })
        })
        .collect()
}

fn remote_licenses(
    repo_url: &Option<String>,
    local: &Vec<PathBuf>,
    search_remote: &SearchRemote,
) -> anyhow::Result<Vec<url::Url>> {
    let repo_url = match repo_url {
        Some(url) => url,
        None => return Ok(Vec::new()),
    };
    match (local.len(), search_remote) {
        (0, SearchRemote::Auto) | (_, SearchRemote::Always) => {
            Ok(remote::license_file_urls(repo_url)?.collect())
        }
        _ => Ok(Vec::new()),
    }
}
