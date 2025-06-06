use std::path::{Path, PathBuf};

use crate::ConfigError;

pub const CONFIG_NAMES: [&str; 4] = [
    ".edlintrc.json",
    ".edlintrc",
    ".edlintrc.yaml",
    ".edlintrc.yml",
];

pub fn find_config(cwd: &Path) -> Result<PathBuf, ConfigError> {
    let mut current_dir = cwd.to_path_buf();

    loop {
        for name in CONFIG_NAMES {
            let config_path = current_dir.join(name);
            if config_path.exists() && config_path.is_file() {
                return Ok(config_path);
            }
        }

        if !current_dir.pop() {
            // Достигли корня файловой системы
            return Err(ConfigError::ConfigNotFound);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_config() {
        let mut crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        crate_root.push("src/fixtures/project-root");
        let config_path = find_config(crate_root.as_path()).unwrap();
        assert_eq!(
            config_path,
            crate_root.join(".edlintrc.yaml"),
            "Config find file in the project root"
        );
    }

    #[test]
    fn test_find_config_not_found() {
        let mut crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        crate_root.push("src/fixtures");
        let config_path = find_config(crate_root.as_path());
        assert!(config_path.is_err(), "Config not found in the root");
    }

    #[test]
    fn test_find_config_in_parent_dir() {
        let mut crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        crate_root.push("src/fixtures/project-root/subdir");
        let config_path = find_config(crate_root.as_path()).unwrap();
        crate_root.pop();
        assert_eq!(
            config_path,
            crate_root.join(".edlintrc.yaml"),
            "Config found in the parent dir"
        );
    }
}
