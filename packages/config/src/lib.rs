mod config;
mod config_schema;
mod error;
mod find_config;
mod schema_to_config;
mod user;

pub use config::*;
pub use config_schema::ConfigSchema;
pub use error::ConfigError;
pub use find_config::find_config;
use schema_to_config::{ConfigDefaults, convert_schema_to_config};
pub use user::User;

use std::{fs, path::Path};

/// Загружает конфигурацию из файла.
pub fn load_config(path: &Path) -> Result<Config, ConfigError> {
    let content = fs::read_to_string(path)?;

    let extension = path.extension().unwrap_or_default();
    let schema = if extension == "yml" || extension == "yaml" {
        serde_yaml::from_str(&content)?
    } else {
        serde_json::from_str(&content)?
    };

    convert_schema_to_config(schema, &ConfigDefaults::default())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_load_config() {
        let crate_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/fixtures/ed-small-yaml");
        let config_path = find_config(&crate_root).unwrap();
        let config = load_config(&config_path).unwrap();
        assert_eq!(config.root.name, "root");
        assert_eq!(config.root.children.len(), 3);
    }

    #[test]
    fn test_user() {
        let mut user = User::new("John".to_string(), 20);

        let user2 = &mut user;

        assert_eq!(user2.is_adult(), true);

        user2.update_name("Jane".to_string());

        assert_eq!(user.get_name(), "Jane");
    }
}
