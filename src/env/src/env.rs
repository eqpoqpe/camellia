// APP_ENV
// Runtime environment variable reading with fallback to build configuration

use std::env;

/// Get the application environment at runtime.
/// First checks for APP_ENV environment variable, then falls back to build-time configuration:
/// - If debug_assertions is enabled, returns "development"
/// - If debug_assertions is disabled, returns "production"
pub fn get_app_env() -> String {
    env::var("APP_ENV").unwrap_or_else(|_| {
        #[cfg(debug_assertions)]
        {
            "development".to_string()
        }
        #[cfg(not(debug_assertions))]
        {
            "production".to_string()
        }
    })
}

/// Function version of get_app_env for easier usage
#[inline]
pub fn app_env() -> String {
    get_app_env()
}

/// Check if the current environment is development
pub fn is_development() -> bool {
    app_env() == "development"
}

/// Check if the current environment is production
pub fn is_production() -> bool {
    app_env() == "production"
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get_app_env_function() {
        let env_val = get_app_env();
        assert!(env_val == "development" || env_val == "production");
        assert!(!env_val.is_empty());
    }

    #[test]
    fn test_app_env_function() {
        let env_val = app_env();
        assert!(env_val == "development" || env_val == "production");
        assert!(!env_val.is_empty());
    }

    #[test]
    fn test_app_env_with_explicit_env_var() {
        // Test with explicit environment variable
        unsafe {
            env::set_var("APP_ENV", "testing");
        }
        assert_eq!(app_env(), "testing");

        // Clean up
        unsafe {
            env::remove_var("APP_ENV");
        }
    }

    #[test]
    fn test_app_env_debug_mode() {
        // Remove any existing APP_ENV to test fallback
        unsafe {
            env::remove_var("APP_ENV");
        }

        #[cfg(debug_assertions)]
        {
            assert_eq!(app_env(), "development");
        }
    }

    #[test]
    fn test_app_env_release_mode() {
        // Remove any existing APP_ENV to test fallback
        unsafe {
            env::remove_var("APP_ENV");
        }

        #[cfg(not(debug_assertions))]
        {
            assert_eq!(app_env(), "production");
        }
    }

    #[test]
    fn test_is_development() {
        // Test with development environment
        unsafe {
            env::set_var("APP_ENV", "development");
        }
        assert!(is_development());
        assert!(!is_production());

        // Clean up
        unsafe {
            env::remove_var("APP_ENV");
        }
    }

    #[test]
    fn test_is_production() {
        // Test with production environment
        unsafe {
            env::set_var("APP_ENV", "production");
        }
        assert!(is_production());
        assert!(!is_development());

        // Clean up
        unsafe {
            env::remove_var("APP_ENV");
        }
    }

    #[test]
    fn test_environment_mutually_exclusive() {
        // Ensure development and production are mutually exclusive
        assert!(is_development() != is_production());
    }
}
