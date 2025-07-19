//! Camellia - A utility library usage by Landmap.
//!
//! This library provides optional modules for environment management, configuration,
//! and JWT authentication based on feature flags.

/// Environment management utilities
///
/// This module provides environment detection and management functionality.
/// It includes utilities to determine the current application environment
/// (development, production, etc.) and access environment-specific configurations.
///
/// # Available functions:
/// - `app_env()`: Get the current application environment
/// - `is_development()`: Check if running in development mode
/// - `is_production()`: Check if running in production mode
///
/// # Example:
/// ```rust
/// use landmap_camellia::env::{app_env, is_development};
///
/// if is_development() {
///     println!("Running in development mode");
/// }
/// ```
#[cfg(feature = "env")]
pub mod env {
    pub use camellia_env::{app_env, is_development, is_production};
}

/// Configuration management utilities
///
/// This module provides comprehensive configuration management for applications.
/// It includes functionality for loading, compiling, and initializing configuration
/// settings from various sources including environment variables, files, and defaults.
///
/// # Available functions:
/// - `compile_config()`: Compile configuration from various sources
/// - `init_config()`: Initialize configuration with default settings
/// - `UseConfigOptions`: Configuration options for customizing behavior
///
/// # Features:
/// - Environment-aware configuration loading
/// - Support for multiple configuration formats
/// - Runtime configuration validation
/// - Integration with environment detection
///
/// # Example:
/// ```rust
/// use landmap_camellia::config::{init_config, UseConfigOptions};
///
/// let options = UseConfigOptions::builder()
///     .schema("myapp")
///     .build();
/// // To use init_config, you would need a struct that implements Deserialize
/// // let config: MyConfig = init_config(options)?;
/// ```
#[cfg(feature = "config")]
pub mod config {
    pub use camellia_config_ext::{
        ConfigBuilderExt, UseConfigOptions, UseConfigOptionsBuilder, init_config,
    };
}
