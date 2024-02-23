use std::path::Path;

pub const WANT_PARSERS: &[&str] = &["lua", "blueprint", "markdown"];

pub fn remove_all(path: &Path) {
    if path.is_file() {
        std::fs::remove_file(path).unwrap();
        return;
    }

    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            remove_all(&path);
        } else {
            std::fs::remove_file(&path).unwrap();
        }
    }

    std::fs::remove_dir(path).unwrap();
}

pub fn setup(test_dir: &Path) {
    if test_dir.exists() {
        remove_all(test_dir)
    }

    std::fs::create_dir_all(test_dir).unwrap();
    std::fs::write(test_dir.join("wanted-parsers.txt"), WANT_PARSERS.join("\n")).unwrap();
}
