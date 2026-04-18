pub mod local;
pub mod output;
pub mod remote;

pub fn is_license(keywords: &[String], file_name: &str) -> bool {
    let file_name = file_name.to_lowercase();
    keywords
        .iter()
        .map(|word| word.to_lowercase())
        .any(|word| file_name.contains(&word))
}
