use crate::Lint;
use crate::lint::{Level, Report};
use crate::local::Local;
use crate::package::Package;
use std::collections::HashSet;

pub fn missing_or_unexpected(
    dependencies: &[Package],
    licenses: &[Local],
) -> (Vec<Report>, Vec<Report>) {
    let expected: HashSet<_> = dependencies.iter().map(|p| p.name.clone()).collect();
    let found: HashSet<_> = licenses.iter().map(|l| l.package.clone()).collect();

    let missing = expected
        .difference(&found)
        .cloned()
        .map(|item| Report {
            lint: Lint::Missing,
            level: Level::Error,
            item,
        })
        .collect();

    let unexpected = found
        .difference(&expected)
        .flat_map(|p| {
            licenses
                .iter()
                .filter(|l| l.package == *p)
                .map(|l| l.file_name())
        })
        .map(|item| Report {
            lint: Lint::Unexpected,
            level: Level::Info,
            item,
        })
        .collect();

    (missing, unexpected)
}
