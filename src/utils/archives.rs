use std::{fs::File, io::BufReader, path::Path};

use bzip2::read::BzDecoder;
use flate2::read::GzDecoder;
use tar::{Archive, Builder};
use xz::{read::XzDecoder, write::XzEncoder};

use super::PATHS;

pub fn extract_tar_gz(archive_path: &Path, extract_dir: &Path) -> anyhow::Result<()> {
    let tar_file = File::open(archive_path)?;
    let buf_reader = BufReader::new(tar_file);
    let gz_decoder = GzDecoder::new(buf_reader);
    let mut tar_archive = Archive::new(gz_decoder);

    tar_archive.unpack(extract_dir)?;
    Ok(())
}

pub fn extract_tar_bz2(archive_path: &Path, extract_dir: &Path) -> anyhow::Result<()> {
    let tar_file = File::open(archive_path)?;
    let buf_reader = BufReader::new(tar_file);
    let bz2_decoder = BzDecoder::new(buf_reader);
    let mut tar_archive = Archive::new(bz2_decoder);

    tar_archive.unpack(extract_dir)?;
    Ok(())
}

pub fn extract_tar_xz(archive_path: &Path, extract_dir: &Path) -> anyhow::Result<()> {
    let tar_file = File::open(archive_path)?;
    let buf_reader = BufReader::new(tar_file);
    let xz_decoder = XzDecoder::new(buf_reader);
    let mut tar_archive = Archive::new(xz_decoder);
    tar_archive.unpack(extract_dir)?;
    Ok(())
}

pub fn create_tar_xz(ouput: &Path, input_paths: &[&Path]) -> anyhow::Result<()> {
    let tar_xz_file = File::create(ouput)?;
    let xz_encoder = XzEncoder::new(tar_xz_file, 9);
    let mut tar_archive = Builder::new(xz_encoder);

    for path in input_paths {
        append_to_archive(&mut tar_archive, path)?;
    }

    Ok(())
}

fn append_to_archive(
    tar_archive: &mut Builder<XzEncoder<File>>,
    path: &Path,
) -> anyhow::Result<()> {
    if path.is_file() {
        let mut file = File::open(path)?;
        let name = path.strip_prefix(&PATHS.ts_parsers).unwrap();
        tar_archive.append_file(name, &mut file)?;
        return Ok(());
    }

    for entry in path.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        append_to_archive(tar_archive, &path)?;
    }
    Ok(())
}
