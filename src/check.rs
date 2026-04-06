use crate::Arguments;
use crate::local::Local;
use crate::package::Package;
use rayon::prelude::*;
use spdx::{LicenseId, detection::scan::Scanner};
use std::collections::HashSet;
use std::path::Path;

pub fn check(args: &Arguments) -> anyhow::Result<()> {
    let dependencies: Vec<_> =
        crate::package::dependencies(&args.project_directory, &args.excluded)?.collect();
    let licenses = crate::local::output_folder_licenses(&args.output_directory);
    let (missing, unexpected) = missing_or_unexpected_licenses(&dependencies, &licenses);
    let licenses = identified_licenses(&licenses)?;
    let unknown = unknown_license_types(&licenses);
    let copy_left = copy_left_licenses(&licenses);

    if !missing.is_empty() {
        println!(
            "{} dependencies missing licenses: {}",
            missing.len(),
            missing.join(", ")
        );
    }

    if !unexpected.is_empty() {
        println!(
            "{} unused dependency licenses found in output folder: {}",
            unexpected.len(),
            unexpected.join(", ")
        );
    }

    if !unknown.is_empty() {
        println!(
            "{} unknown license types: {}",
            unknown.len(),
            unknown.join(", ")
        );
    }

    if !copy_left.is_empty() {
        println!(
            "{} maybe copy-left licenses found: {}",
            copy_left.len(),
            copy_left.join(", ")
        );
    }

    Ok(())
}

fn missing_or_unexpected_licenses(
    dependencies: &[Package],
    licenses: &[Local],
) -> (Vec<String>, Vec<String>) {
    let expected: HashSet<_> = dependencies.iter().map(|p| p.name.clone()).collect();
    let found: HashSet<_> = licenses.iter().map(|l| l.package.clone()).collect();
    let mut missing: Vec<_> = expected.difference(&found).cloned().collect();
    let mut unexpected: Vec<_> = found.difference(&expected).cloned().collect();
    missing.sort();
    unexpected.sort();
    (missing, unexpected)
}

fn identified_licenses(licenses: &'_ [Local]) -> anyhow::Result<Vec<IdentifiedLicense<'_>>> {
    let store = spdx::detection::Store::load_inline()?;
    let scanner = spdx::detection::scan::Scanner::new(&store);
    licenses
        .par_iter()
        .map(|license| identify_license(&scanner, &license))
        .collect()
}

fn unknown_license_types(licenses: &[IdentifiedLicense]) -> Vec<String> {
    licenses
        .iter()
        .filter(|l| l.id().is_none())
        .map(|l| l.license.location.to_string_lossy().to_string())
        .collect()
}

fn copy_left_licenses(licenses: &[IdentifiedLicense]) -> Vec<String> {
    licenses
        .iter()
        .filter(|l| l.id().map(LicenseId::is_copyleft).unwrap_or(false))
        .map(|l| l.license.location.to_string_lossy().to_string())
        .collect()
}

struct IdentifiedLicense<'a> {
    license: &'a Local,
    id_from_name: Option<LicenseId>,
    id_from_content: Option<LicenseId>,
}

impl IdentifiedLicense<'_> {
    fn id(&self) -> Option<LicenseId> {
        self.id_from_content
            .clone()
            .or_else(|| self.id_from_name.clone())
    }
}

fn identify_license<'a>(
    scanner: &Scanner,
    license: &'a Local,
) -> anyhow::Result<IdentifiedLicense<'a>> {
    Ok(IdentifiedLicense {
        id_from_name: id_from_name(&license.location),
        id_from_content: scanner
            .scan(&std::fs::read_to_string(&license.location)?.into())
            .license
            .and_then(|license| spdx::license_id(license.name)),
        license,
    })
}

fn id_from_name(path: &Path) -> Option<LicenseId> {
    // slightly arbitrarily preferring earlier words, and more precise names
    path.file_name()?
        .to_str()?
        .split('-')
        .skip(1) // package name
        .flat_map(possible_ids_from_word)
        .next()
}

fn possible_ids_from_word(word: &str) -> impl Iterator<Item = LicenseId> {
    let precise = spdx::license_id(word).into_iter();
    let imprecise = spdx::imprecise_license_id(word).map(|(id, _)| id);
    precise.chain(imprecise)
}
