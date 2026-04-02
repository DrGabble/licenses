pub fn is_license(file_name: &str) -> bool {
    let names = [
        "license",
        "license-apache",
        "license-mit",
        "license-apache",
        "license-zlib",
        "license-cc0",
        "copying",
        "authors",
    ];
    let file_types = ["md", "txt", "apache2", "mit"];
    match file_name.split_once('.') {
        Some((prefix, suffix)) => {
            names.contains(&prefix.to_lowercase().as_str())
                && file_types.contains(&suffix.to_lowercase().as_str())
        }
        None => names.contains(&file_name.to_lowercase().as_str()),
    }
}
