use super::types::{RegistryConfig, RegistryTarget};


// Embedded at compile time: changing the file requires a rebuild.
const REGISTRY: &str = include_str!("../../targets/registry.toml");


pub(crate) fn registry_targets() -> Result<Vec<RegistryTarget>, toml::de::Error> {
    toml::from_str::<RegistryConfig>(REGISTRY).map(|config| config.registry)
}