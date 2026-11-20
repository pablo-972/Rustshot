use super::types::{FileTarget, FilesConfig};


// Embedded at compile time: changing the file requires a rebuild.
const FILES: &str = include_str!("../../targets/files-windows.toml");


pub(crate) fn file_targets() -> Result<Vec<FileTarget>, toml::de::Error> {
    toml::from_str::<FilesConfig>(FILES).map(|config| config.files)
}