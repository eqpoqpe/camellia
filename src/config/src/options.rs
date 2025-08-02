/// This module defines the options for using the configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct UseConfigOptions {
    /// Optional schema to use for the configuration.
    /// If `None`, the default schema will be used.
    pub schema: Option<String>,
    /// If true, environment variables will not be used to override the configuration.
    pub disable_env: bool,
}

impl UseConfigOptions {
    /// Creates a new `UseConfigOptions` with default values.
    pub fn new() -> Self {
        UseConfigOptions {
            schema: None,
            disable_env: false,
        }
    }

    /// Creates a builder for `UseConfigOptions`.
    pub fn builder() -> UseConfigOptionsBuilder {
        UseConfigOptionsBuilder::new()
    }

    /// Sets the schema to be used for the configuration.
    pub fn with_schema(mut self, schema: &str) -> Self {
        self.schema = Some(schema.to_string());
        self
    }

    /// Sets the schema to be used for the configuration from an optional string.
    pub fn with_schema_opt(mut self, schema: Option<String>) -> Self {
        self.schema = schema;
        self
    }

    /// Disables environment variable overrides for the configuration.
    pub fn with_disable_env(mut self) -> Self {
        self.disable_env = true;
        self
    }

    /// Sets whether to disable environment variable overrides.
    pub fn with_env_disabled(mut self, disable: bool) -> Self {
        self.disable_env = disable;
        self
    }
}

impl Default for UseConfigOptions {
    fn default() -> Self {
        Self::new()
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

    /// Sets the schema from an optional value.
    pub fn schema_opt(mut self, schema: Option<String>) -> Self {
        self.schema = schema;
        self
    }

    /// Disables environment variable overrides for the configuration.
    pub fn disable_env(mut self) -> Self {
        self.disable_env = true;
        self
    }

    /// Sets whether to disable environment variable overrides.
    pub fn env_disabled(mut self, disable: bool) -> Self {
        self.disable_env = disable;
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
    fn test_new_creates_default_options() {
        let options = UseConfigOptions::new();
        assert_eq!(options.schema, None);
        assert_eq!(options.disable_env, false);
    }

    #[test]
    fn test_with_schema() {
        let options = UseConfigOptions::new().with_schema("test_schema");
        assert_eq!(options.schema, Some("test_schema".to_string()));
        assert_eq!(options.disable_env, false);
    }

    #[test]
    fn test_with_disable_env() {
        let options = UseConfigOptions::new().with_disable_env();
        assert_eq!(options.disable_env, true);
        assert_eq!(options.schema, None);
    }

    #[test]
    fn test_builder_pattern() {
        let options = UseConfigOptions::builder()
            .schema("my_schema")
            .disable_env()
            .build();

        assert_eq!(options.schema, Some("my_schema".to_string()));
        assert_eq!(options.disable_env, true);
    }

    #[test]
    fn test_builder_with_optional_schema() {
        let options = UseConfigOptions::builder()
            .schema_opt(Some("optional_schema".to_string()))
            .build();

        assert_eq!(options.schema, Some("optional_schema".to_string()));
        assert_eq!(options.disable_env, false);
    }

    #[test]
    fn test_builder_default() {
        let options = UseConfigOptions::builder().build();
        assert_eq!(options.schema, None);
        assert_eq!(options.disable_env, false);
    }

    #[test]
    fn test_builder_env_disabled_flag() {
        let options = UseConfigOptions::builder().env_disabled(true).build();

        assert_eq!(options.disable_env, true);

        let options2 = UseConfigOptions::builder().env_disabled(false).build();

        assert_eq!(options2.disable_env, false);
    }

    #[test]
    fn test_chaining_methods() {
        let options = UseConfigOptions::new()
            .with_schema("chained_schema")
            .with_disable_env();

        assert_eq!(options.schema, Some("chained_schema".to_string()));
        assert_eq!(options.disable_env, true);
    }

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
