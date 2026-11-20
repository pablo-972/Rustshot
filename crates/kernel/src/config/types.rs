use serde::Deserialize;


// The whole registry targets file.
#[derive(Debug, Clone, Deserialize)]
pub(super) struct RegistryConfig {
    pub registry: Vec<RegistryTarget>,
}


// One registry key to capture, walked recursively.
// Other fields in the file (description, os) are ignored.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct RegistryTarget {
    pub key: String,
}


// The whole file targets file.
#[derive(Debug, Clone, Deserialize)]
pub(super) struct FilesConfig {
    pub files: Vec<FileTarget>,
}


// One file or folder to capture.
// Other fields in the file (description) are ignored.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct FileTarget {
    pub path: String,

    #[serde(default)]
    pub recursive: bool,

    #[serde(default = "default_hash")]
    pub hash: bool,
}


fn default_hash() -> bool {
    true
}