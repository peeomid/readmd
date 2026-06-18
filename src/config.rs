use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{ReadmdError, Result};
use crate::theme::{Theme, builtin_themes};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Config {
    pub default_theme: String,
    #[serde(default = "default_style_name")]
    pub default_style: String,
    #[serde(default = "default_generated_by_readmd")]
    pub generated_by_readmd: bool,
    #[serde(default)]
    pub themes: BTreeMap<String, Theme>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Default)]
struct ConfigFile {
    #[serde(default)]
    default_theme: Option<String>,
    #[serde(default)]
    default_style: Option<String>,
    #[serde(default)]
    generated_by_readmd: Option<bool>,
    #[serde(default)]
    themes: BTreeMap<String, Theme>,
}

pub fn default_config() -> Config {
    Config {
        default_theme: "paper".to_string(),
        default_style: default_style_name(),
        generated_by_readmd: default_generated_by_readmd(),
        themes: builtin_themes(),
    }
}

fn default_style_name() -> String {
    "editorial".to_string()
}

fn default_generated_by_readmd() -> bool {
    true
}

pub fn default_config_toml() -> String {
    print_config_toml(&default_config())
}

pub fn print_config_toml(config: &Config) -> String {
    toml::to_string_pretty(config).expect("config should serialize to TOML")
}

pub fn config_from_str(input: &str) -> Result<Config> {
    let parsed: ConfigFile = toml::from_str(input).map_err(|source| ReadmdError::ParseToml {
        path: PathBuf::from("<memory>"),
        source,
    })?;
    Ok(merge_config(default_config(), parsed.into_config()))
}

pub fn load_config(explicit_path: Option<&Path>) -> Result<Config> {
    let base = default_config();

    if let Some(path) = explicit_path {
        return load_config_file(path).map(|user| merge_config(base, user));
    }

    for candidate in lookup_paths() {
        if candidate.exists() {
            return load_config_file(&candidate).map(|user| merge_config(base, user));
        }
    }

    Ok(base)
}

pub fn load_config_file(path: &Path) -> Result<Config> {
    let raw = fs::read_to_string(path).map_err(|source| ReadmdError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;
    let parsed: ConfigFile = toml::from_str(&raw).map_err(|source| ReadmdError::ParseToml {
        path: path.to_path_buf(),
        source,
    })?;
    Ok(merge_config(default_config(), parsed.into_config()))
}

pub fn merge_config(mut base: Config, other: Config) -> Config {
    if !other.default_theme.is_empty() {
        base.default_theme = other.default_theme;
    }
    if !other.default_style.is_empty() {
        base.default_style = other.default_style;
    }
    base.generated_by_readmd = other.generated_by_readmd;
    for (name, theme) in other.themes {
        base.themes.insert(name, theme);
    }
    base
}

pub fn lookup_paths() -> Vec<PathBuf> {
    let mut paths = vec![PathBuf::from("readmd.toml")];
    if let Some(home) = dirs::home_dir() {
        paths.push(home.join(".config/readmd/config.toml"));
    }
    paths
}

impl ConfigFile {
    fn into_config(self) -> Config {
        Config {
            default_theme: self.default_theme.unwrap_or_default(),
            default_style: self.default_style.unwrap_or_default(),
            generated_by_readmd: self
                .generated_by_readmd
                .unwrap_or_else(default_generated_by_readmd),
            themes: self.themes,
        }
    }
}

impl Config {
    pub fn load(path: Option<&Path>) -> Result<Self> {
        load_config(path)
    }

    pub fn theme(&self, name: &str) -> Option<&Theme> {
        self.themes.get(name)
    }

    pub fn theme_with_style(&self, theme_name: &str, style_name: &str) -> Option<&Theme> {
        let combined = format!("{theme_name}-{style_name}");
        self.themes
            .get(&combined)
            .or_else(|| self.themes.get(theme_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_toml_has_paper_and_night() {
        let toml = default_config_toml();
        assert!(toml.contains("default_theme = \"paper\""));
        assert!(toml.contains("default_style = \"editorial\""));
        assert!(toml.contains("generated_by_readmd = true"));
        assert!(toml.contains("[themes.paper]"));
        assert!(toml.contains("[themes.night]"));
    }

    #[test]
    fn merge_config_can_add_custom_theme() {
        let user = config_from_str(
            r##"
default_theme = "custom"
default_style = "notebook"
generated_by_readmd = false

[themes.custom]
name = "Custom"

[themes.custom.page]
background = "#ffffff"
padding = "24px"
mobile_padding = "12px"
            "##,
        )
        .expect("parse user config");
        let merged = merge_config(default_config(), user);

        assert_eq!(merged.default_theme, "custom");
        assert_eq!(merged.default_style, "notebook");
        assert!(!merged.generated_by_readmd);
        assert!(merged.themes.contains_key("custom"));
        assert!(merged.themes.contains_key("paper"));
        assert!(merged.themes.contains_key("night"));
    }

    #[test]
    fn lookup_paths_include_local_then_home() {
        let paths = lookup_paths();
        assert_eq!(paths[0], PathBuf::from("readmd.toml"));
    }
}
