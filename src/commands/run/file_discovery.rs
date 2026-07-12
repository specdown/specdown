use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::parsers;
use crate::runner::Error;

/// Builds the complete, deduplicated list of spec files to run.
///
/// When `follow_links` is `false`, returns `initial_files` unchanged (no
/// filesystem access, no behavioural change from before this feature
/// existed). Otherwise performs a depth-first, pre-order traversal: each
/// file is read, its local Markdown links are extracted and resolved
/// relative to that file's own parent directory, and each resolved link is
/// recursively visited before moving on to the next link, in document
/// order. Visited files are tracked by canonicalized absolute path, so
/// link cycles terminate naturally and every file is included exactly
/// once, in first-discovery order.
pub fn build_file_list(
    initial_files: &[PathBuf],
    base_dir: &Path,
    follow_links: bool,
) -> Result<Vec<PathBuf>, Error> {
    if !follow_links {
        return Ok(initial_files.to_vec());
    }

    let canonical_base = fs::canonicalize(base_dir).unwrap_or_else(|_| base_dir.to_path_buf());
    let mut visited: HashSet<PathBuf> = HashSet::new();
    let mut ordered: Vec<PathBuf> = Vec::new();

    for file in initial_files {
        visit(file, base_dir, &canonical_base, &mut visited, &mut ordered)?;
    }

    Ok(ordered)
}

fn visit(
    file: &Path,
    dir: &Path,
    canonical_base: &Path,
    visited: &mut HashSet<PathBuf>,
    ordered: &mut Vec<PathBuf>,
) -> Result<(), Error> {
    let absolute = if file.has_root() {
        file.to_path_buf()
    } else {
        dir.join(file)
    };
    let canonical = fs::canonicalize(&absolute).map_err(|e| Error::LinkedFileUnreadable {
        path: file.display().to_string(),
        message: e.to_string(),
    })?;

    if !visited.insert(canonical.clone()) {
        // Already seen — a link cycle or a diamond dependency. Stop here
        // rather than visiting (and running) the file again.
        return Ok(());
    }

    // Store a path relative to the discovery root when possible, so
    // printed output (and doc `verify()` blocks) shows stable names like
    // "a.md" instead of a non-deterministic absolute temp-dir path.
    let display_path = canonical
        .strip_prefix(canonical_base)
        .map_or_else(|_| canonical.clone(), Path::to_path_buf);
    ordered.push(display_path);

    let contents = fs::read_to_string(&canonical).map_err(|e| Error::LinkedFileUnreadable {
        path: file.display().to_string(),
        message: e.to_string(),
    })?;

    let parent_dir = canonical.parent().unwrap_or(dir).to_path_buf();

    for link in parsers::find_links(&contents)? {
        if let Some(link_path) = local_markdown_link(&link) {
            visit(&link_path, &parent_dir, canonical_base, visited, ordered)?;
        }
    }

    Ok(())
}

/// Decides whether a raw link URL should be followed as a local spec file.
///
/// Skips absolute/external URLs (containing `://`), `mailto:` links,
/// anchor-only (`#...`) links, and non-`.md` targets. Strips any trailing
/// `#fragment`/`?query` before checking the extension.
fn local_markdown_link(raw: &str) -> Option<PathBuf> {
    if raw.contains("://") || raw.starts_with("mailto:") || raw.starts_with('#') {
        return None;
    }

    let without_fragment = raw.split(['#', '?']).next().unwrap_or(raw);
    if without_fragment.is_empty() {
        return None;
    }

    let path = PathBuf::from(without_fragment);
    path.extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
        .then_some(path)
}

#[cfg(test)]
mod tests {
    use super::build_file_list;
    use std::fs;
    use std::path::PathBuf;

    fn write(dir: &std::path::Path, name: &str, content: &str) {
        fs::write(dir.join(name), content).expect("failed to write fixture file");
    }

    #[test]
    fn returns_initial_files_unchanged_when_follow_links_is_false() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let files = vec![PathBuf::from("a.md"), PathBuf::from("b.md")];

        let result = build_file_list(&files, dir.path(), false).expect("should succeed");

        assert_eq!(result, files);
    }

    #[test]
    fn follows_a_single_local_link() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        write(dir.path(), "a.md", "[link](b.md)");
        write(dir.path(), "b.md", "no links here");

        let files = vec![PathBuf::from("a.md")];
        let result = build_file_list(&files, dir.path(), true).expect("should succeed");

        assert_eq!(result, vec![PathBuf::from("a.md"), PathBuf::from("b.md")]);
    }

    #[test]
    fn follows_links_recursively() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        write(dir.path(), "a.md", "[link](b.md)");
        write(dir.path(), "b.md", "[link](c.md)");
        write(dir.path(), "c.md", "no links here");

        let files = vec![PathBuf::from("a.md")];
        let result = build_file_list(&files, dir.path(), true).expect("should succeed");

        assert_eq!(
            result,
            vec![
                PathBuf::from("a.md"),
                PathBuf::from("b.md"),
                PathBuf::from("c.md"),
            ]
        );
    }

    #[test]
    fn breaks_link_cycles_and_visits_each_file_once() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        write(dir.path(), "a.md", "[link](b.md)");
        write(dir.path(), "b.md", "[link](a.md)");

        let files = vec![PathBuf::from("a.md")];
        let result = build_file_list(&files, dir.path(), true).expect("should succeed");

        assert_eq!(result, vec![PathBuf::from("a.md"), PathBuf::from("b.md")]);
    }

    #[test]
    fn ignores_non_local_and_non_markdown_links() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        write(
            dir.path(),
            "a.md",
            "[ext](https://example.com/x.md) [mail](mailto:a@b.com) [anchor](#section) [img](image.png)",
        );

        let files = vec![PathBuf::from("a.md")];
        let result = build_file_list(&files, dir.path(), true).expect("should succeed");

        assert_eq!(result, vec![PathBuf::from("a.md")]);
    }

    #[test]
    fn resolves_links_relative_to_the_containing_file_directory() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        fs::create_dir(dir.path().join("sub")).expect("failed to create sub dir");
        write(dir.path(), "a.md", "[link](sub/b.md)");
        write(&dir.path().join("sub"), "b.md", "[link](../c.md)");
        write(dir.path(), "c.md", "no links here");

        let files = vec![PathBuf::from("a.md")];
        let result = build_file_list(&files, dir.path(), true).expect("should succeed");

        assert_eq!(
            result,
            vec![
                PathBuf::from("a.md"),
                PathBuf::from("sub/b.md"),
                PathBuf::from("c.md"),
            ]
        );
    }

    #[test]
    fn errors_on_a_broken_link() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        write(dir.path(), "a.md", "[missing](nope.md)");

        let files = vec![PathBuf::from("a.md")];
        let result = build_file_list(&files, dir.path(), true);

        assert!(result.is_err(), "expected an error for a broken link");
    }

    #[test]
    fn deduplicates_a_diamond_dependency() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        write(dir.path(), "a.md", "[b](b.md) [c](c.md)");
        write(dir.path(), "b.md", "[d](d.md)");
        write(dir.path(), "c.md", "[d](d.md)");
        write(dir.path(), "d.md", "no links here");

        let files = vec![PathBuf::from("a.md")];
        let result = build_file_list(&files, dir.path(), true).expect("should succeed");

        assert_eq!(
            result,
            vec![
                PathBuf::from("a.md"),
                PathBuf::from("b.md"),
                PathBuf::from("d.md"),
                PathBuf::from("c.md"),
            ]
        );
    }
}
