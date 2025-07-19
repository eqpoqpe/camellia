/// This module defines the options for using the configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct UseConfigOptions {
    /// Optional schema to use for the configuration.
    /// If `None`, the default schema will be used.
    pub schema: Option<String>,
    /// If true, environment variables will not be used to override the configuration.
    pub disable_env: bool,
}

impl Default for UseConfigOptions {
    fn default() -> Self {
        UseConfigOptions {
            schema: None,
            disable_env: false,
        }
    }
}

/// Builder for `UseConfigOptions`.
#[derive(Debug, Clone)]
pub struct UseConfigOptionsBuilder {
    schema: Option<String>,
    disable_env: bool,
}

impl UseConfigOptionsBuilder {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        UseConfigOptionsBuilder {
            schema: None,
            disable_env: false,
        }
    }

    /// Sets the schema to be used for the configuration.
    pub fn schema<S: Into<String>>(mut self, schema: S) -> Self {
        self.schema = Some(schema.into());
        self
    }

    /// Disables environment variable overrides for the configuration.
    pub fn disable_env(mut self) -> Self {
        self.disable_env = true;
        self
    }

    /// Builds the `UseConfigOptions` instance.
    pub fn build(self) -> UseConfigOptions {
        UseConfigOptions {
            schema: self.schema,
            disable_env: self.disable_env,
        }
    }
}

impl Default for UseConfigOptionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_trait() {
        let options = UseConfigOptions::default();
        assert_eq!(options.schema, None);
        assert_eq!(options.disable_env, false);
    }

    #[test]
    fn test_builder_default_trait() {
        let builder = UseConfigOptionsBuilder::default();
        let options = builder.build();
        assert_eq!(options.schema, None);
        assert_eq!(options.disable_env, false);
    }
}
