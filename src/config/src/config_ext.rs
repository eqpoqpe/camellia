use crate::options::UseConfigOptions;
use camellia_env::app_env;
use config::{ConfigBuilder, ConfigError, Environment, File, builder};

pub trait ConfigBuilderExt {
    fn add_config_sources(self, options: UseConfigOptions) -> Self;
}

impl ConfigBuilderExt for ConfigBuilder<builder::DefaultState> {
    fn add_config_sources(self, options: UseConfigOptions) -> Self {
        // Start with the base config file
        let schema = options.schema.as_deref().unwrap_or("default");
        let env = app_env();

        let mut cb = if schema == "default" {
            self.add_source(File::with_name("config/default"))
        } else {
            self.add_source(File::with_name(&format!("config/{}/default", schema)))
        };

        // Add environment-specific config (skip for production)
        if env != "production" {
            let env_config_path = if schema == "default" {
                format!("config/{}", env)
            } else {
                format!("config/{}/{}", schema, env)
            };

            cb = cb.add_source(File::with_name(&env_config_path).required(false));
        }

        // Add environment variables if not disabled
        if !options.disable_env {
            cb = cb.add_source(
                Environment::default()
                    .separator("__")
                    .prefix(&schema.to_uppercase()),
            );
        }

        cb
    }
}

/// Environment variables can override the values in Config.toml
pub fn init_config<T: serde::de::DeserializeOwned>(
    options: crate::options::UseConfigOptions,
) -> Result<T, ConfigError> {
    let config = config::Config::builder()
        .add_config_sources(options)
        .build()?;

    config.try_deserialize()
}
