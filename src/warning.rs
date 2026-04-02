use crate::dependency::Dependency;

pub fn print_warnings(deps: &Vec<Dependency>) {
    let total_licenses = deps
        .iter()
        .map(|d| d.local_licenses.len() + d.remote_licenses.len())
        .sum::<usize>();
    let no_licenses: Vec<_> = deps
        .iter()
        .filter(|d| d.local_licenses.is_empty() && d.remote_licenses.is_empty())
        .map(|d| d.name.clone())
        .collect();
    println!(
        "{} licenses found in {} dependencies",
        total_licenses,
        deps.len()
    );
    if !no_licenses.is_empty() {
        println!("no licenses found for {}", no_licenses.join(", "));
    }
}
