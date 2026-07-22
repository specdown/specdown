use std::path::{Path, PathBuf};

use serde::Deserialize;

use super::settings::RunSettings;
use crate::runner::Error;

pub struct LoadedConfig {
    pub settings: RunSettings,
    pub path: Option<PathBuf>,
}

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
#[cfg(test)]
fn load_run_settings(explicit_path: Option<&Path>, cwd: &Path) -> Result<RunSettings, Error> {
    load_config(explicit_path, cwd).map(|config| config.settings)
}

pub fn load_config(explicit_path: Option<&Path>, cwd: &Path) -> Result<LoadedConfig, Error> {
    let (path, error_path, is_explicit): (PathBuf, PathBuf, bool) = if let Some(p) = explicit_path {
        (make_absolute(p, cwd), p.to_path_buf(), true)
    } else {
        let path = cwd.join(DEFAULT_FILE_NAME);
        (path.clone(), path, false)
    };

    let contents = match std::fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound && !is_explicit => {
            return Ok(LoadedConfig {
                settings: RunSettings::default(),
                path: None,
            });
        }
        Err(err) => {
            return Err(Error::ConfigFileLoadFailed {
                path: error_path,
                message: err.to_string(),
            })
        }
    };

    toml::from_str::<ConfigFile>(&contents)
        .map(|config| LoadedConfig {
            settings: config.run,
            path: Some(path),
        })
        .map_err(|err| Error::ConfigFileLoadFailed {
            path: error_path,
            message: err.to_string(),
        })
}

fn make_absolute(path: &Path, cwd: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        cwd.join(path)
    }
}

#[cfg(test)]
mod tests {
    use super::{load_config, load_run_settings};
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
    fn test_resolves_relative_add_path_from_config_directory() {
        let dir = tempfile::tempdir().expect("failed to create a temporary directory");
        let config_path = dir.path().join("nested").join("custom.toml");
        fs::create_dir_all(config_path.parent().expect("config should have a parent"))
            .expect("failed to create config directory");
        fs::write(&config_path, "[run]\nadd_path = [\"bin\"]\n")
            .expect("failed to write config file");

        let loaded =
            load_config(Some(&config_path), dir.path()).expect("expected settings, got an error");

        assert_eq!(Some(config_path), loaded.path);
        assert_eq!(vec!["bin".to_string()], loaded.settings.add_path);
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
