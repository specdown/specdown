use std::path::Path;

/// Builds the `SPECDOWN_*` environment variables passed to spec scripts.
///
/// * `start_dir` - the directory `specdown` was invoked from.
/// * `workspace_dir` - the workspace directory (see `--workspace-dir` /
///   `--temporary-workspace-dir`).
/// * `working_dir` - the directory scripts actually run in (the workspace
///   directory, optionally joined with `--working-dir`).
pub fn build(start_dir: &Path, workspace_dir: &Path, working_dir: &Path) -> Vec<(String, String)> {
    vec![
        ("SPECDOWN_START_DIR".to_string(), path_to_string(start_dir)),
        (
            "SPECDOWN_WORKSPACE_DIR".to_string(),
            path_to_string(workspace_dir),
        ),
        (
            "SPECDOWN_WORKING_DIR".to_string(),
            path_to_string(working_dir),
        ),
    ]
}

fn path_to_string(path: &Path) -> String {
    path.to_str().expect("path must be valid UTF-8").to_string()
}

#[cfg(test)]
mod tests {
    use super::build;
    use std::path::Path;

    #[test]
    fn builds_all_three_specdown_env_vars() {
        let env = build(
            Path::new("/start"),
            Path::new("/workspace"),
            Path::new("/working"),
        );

        assert_eq!(
            env,
            vec![
                ("SPECDOWN_START_DIR".to_string(), "/start".to_string()),
                (
                    "SPECDOWN_WORKSPACE_DIR".to_string(),
                    "/workspace".to_string()
                ),
                ("SPECDOWN_WORKING_DIR".to_string(), "/working".to_string()),
            ]
        );
    }
}
