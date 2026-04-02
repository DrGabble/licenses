pub fn is_license(file_name: &str) -> bool {
    let file_name = file_name.to_lowercase();
    let prefixes = ["license", "copying", "authors", "copyright"];
    prefixes.iter().any(|prefix| file_name.contains(prefix))
}
