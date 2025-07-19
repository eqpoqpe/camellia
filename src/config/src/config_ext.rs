use crate::options::UseConfigOptions;
use camellia_env::app_env;
use config::{ConfigBuilder, Environment, File, builder};

pub trait ConfigBuilderExt {
    /// Adds configuration sources based on the provided options.
    /// This includes a base config file, environment-specific files, and environment variables.
    /// # Arguments:
    /// - `options`: Configuration options that specify schema and environment variable behavior.
    /// # Returns:
    /// A modified `ConfigBuilder` with the added sources.
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
