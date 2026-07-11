use std::path::{Path, PathBuf};

use serde::Deserialize;

use super::settings::RunSettings;
use crate::runner::Error;

/// The default name of the config file looked up in the current directory
/// when `--config` is not given.
const DEFAULT_FILE_NAME: &str = "specdown.toml";

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct ConfigFile {
    #[serde(default)]
    run: RunSettings,
}

/// Loads the `[run]` settings from a `specdown.toml` file.
///
/// If `explicit_path` is given (from `--config`), that file is loaded and it
/// is an error if it doesn't exist or fails to parse. Otherwise `cwd` is
/// searched for a `specdown.toml`; if it isn't there, the defaults
/// (`RunSettings::default()`) are returned rather than an error.
pub fn load_run_settings(explicit_path: Option<&Path>, cwd: &Path) -> Result<RunSettings, Error> {
    let (path, is_explicit): (PathBuf, bool) = match explicit_path {
        Some(p) => (p.to_path_buf(), true),
        None => (cwd.join(DEFAULT_FILE_NAME), false),
    };

    let contents = match std::fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound && !is_explicit => {
            return Ok(RunSettings::default());
        }
        Err(err) => {
            return Err(Error::ConfigFileLoadFailed {
                path,
                message: err.to_string(),
            })
        }
    };

    toml::from_str::<ConfigFile>(&contents)
        .map(|config| config.run)
        .map_err(|err| Error::ConfigFileLoadFailed {
            path,
            message: err.to_string(),
        })
}

#[cfg(test)]
mod tests {
    use super::load_run_settings;
    use crate::commands::run::settings::ExecutorKind;
    use crate::runner::Error;
    use std::fs;

    #[test]
    fn test_returns_defaults_when_no_file_exists_at_the_default_location() {
        let dir = tempfile::tempdir().expect("failed to create a temporary directory");

        let settings =
            load_run_settings(None, dir.path()).expect("expected default settings, got an error");

        assert!(settings.spec_files.is_empty());
        assert_eq!(None, settings.shell_command);
    }

    #[test]
    fn test_errors_when_an_explicit_config_path_does_not_exist() {
        let dir = tempfile::tempdir().expect("failed to create a temporary directory");
        let missing_path = dir.path().join("does-not-exist.toml");

        let result = load_run_settings(Some(&missing_path), dir.path());

        assert!(matches!(result, Err(Error::ConfigFileLoadFailed { .. })));
    }

    #[test]
    fn test_loads_settings_from_the_default_location() {
        let dir = tempfile::tempdir().expect("failed to create a temporary directory");
        fs::write(
            dir.path().join("specdown.toml"),
            r#"
                [run]
                shell_command = "sh -c"
                jobs = 4

                [run.env]
                FOO = "bar"

                [run.executor]
                executor = "container"
                container_image = "bash:5"
            "#,
        )
        .expect("failed to write config file");

        let settings =
            load_run_settings(None, dir.path()).expect("expected settings, got an error");

        assert_eq!(Some("sh -c".to_string()), settings.shell_command);
        assert_eq!(Some(4), settings.jobs);
        assert_eq!(vec!["FOO=bar".to_string()], settings.env);
        assert_eq!(
            Some(ExecutorKind::Container),
            settings.executor_config.executor
        );
        assert_eq!(
            Some("bash:5".to_string()),
            settings.executor_config.container_image
        );
    }

    #[test]
    fn test_loads_multiple_env_vars_from_the_env_table_in_sorted_order() {
        let dir = tempfile::tempdir().expect("failed to create a temporary directory");
        fs::write(
            dir.path().join("specdown.toml"),
            r#"
                [run.env]
                B = "2"
                A = "1"
            "#,
        )
        .expect("failed to write config file");

        let settings =
            load_run_settings(None, dir.path()).expect("expected settings, got an error");

        assert_eq!(vec!["A=1".to_string(), "B=2".to_string()], settings.env);
    }

    #[test]
    fn test_loads_settings_from_an_explicit_path() {
        let dir = tempfile::tempdir().expect("failed to create a temporary directory");
        let config_path = dir.path().join("custom.toml");
        fs::write(
            &config_path,
            r#"
                [run]
                shell_command = "zsh -c"
            "#,
        )
        .expect("failed to write config file");

        let settings = load_run_settings(Some(&config_path), dir.path())
            .expect("expected settings, got an error");

        assert_eq!(Some("zsh -c".to_string()), settings.shell_command);
    }

    #[test]
    fn test_errors_on_an_unknown_key() {
        let dir = tempfile::tempdir().expect("failed to create a temporary directory");
        fs::write(
            dir.path().join("specdown.toml"),
            r#"
                [run]
                shell_comand = "typo'd key"
            "#,
        )
        .expect("failed to write config file");

        let result = load_run_settings(None, dir.path());

        assert!(matches!(result, Err(Error::ConfigFileLoadFailed { .. })));
    }

    #[test]
    fn test_errors_on_malformed_toml() {
        let dir = tempfile::tempdir().expect("failed to create a temporary directory");
        fs::write(dir.path().join("specdown.toml"), "this is not valid toml")
            .expect("failed to write config file");

        let result = load_run_settings(None, dir.path());

        assert!(matches!(result, Err(Error::ConfigFileLoadFailed { .. })));
    }
}
