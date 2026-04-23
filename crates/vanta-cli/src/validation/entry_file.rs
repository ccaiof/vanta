use std::path::Path;

pub fn validate_entry_file(path: &Path) {
    if !path.is_file() {
        panic!("main.vt exists but is not a valid file");
    }
}