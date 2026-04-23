use std::path::{Path, PathBuf};

use walkdir::WalkDir;

pub fn validate_project_context(root: &Path) -> PathBuf {
    let main_file = root.join("main.vt");

    if !main_file.exists() {
        panic!("main.vt was not found in the current Vanta project root");
    }

    let mut extra_main_files = Vec::new();

    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        if path == main_file {
            continue;
        }

        if path.is_file() && path.file_name().and_then(|name| name.to_str()) == Some("main.vt") {
            extra_main_files.push(path.to_path_buf());
        }
    }

    if !extra_main_files.is_empty() {
        let files = extra_main_files
            .iter()
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>()
            .join(", ");

        panic!("multiple main.vt files detected in the current Vanta context: {files}");
    }

    main_file
}
