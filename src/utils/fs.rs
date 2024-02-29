use std::{fs, path::Path};

pub fn copy_all(src: &Path, dst: &Path) -> anyhow::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), &dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn remove_all(path: &Path) -> anyhow::Result<()> {
    if path.is_file() {
        std::fs::remove_file(path)?;
        return Ok(());
    }

    for entry in path.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            remove_all(&path)?;
        } else {
            std::fs::remove_file(&path)?;
        }
    }

    std::fs::remove_dir(path)?;

    Ok(())
}
