use std::fs::File;
use std::io::Cursor;
use std::path::Path;

use reqwest::redirect::Policy;
use reqwest::Client;

use crate::c_println;

/// reference: https://georgik.rocks/how-to-download-binary-file-in-rust-by-reqwest/
pub async fn download_file(url: &str, destination: impl AsRef<Path>) -> anyhow::Result<()> {
    let client = Client::builder().redirect(Policy::limited(5)).build()?;
    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        c_println!(red, "Failed to download file: {}", response.status());
    }

    let mut file = File::create(destination)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_download_file() {
        let url = "https://github.com/MunifTanjim/tree-sitter-lua/archive/9668709211b2e683f27f414454a8b51bf0a6bda1.zip";
        let download_location = std::env::current_dir()
            .unwrap()
            .join("test-assets")
            .join("lua.zip");
        download_file(url, &download_location).await.unwrap();
        assert!(download_location.exists());
        std::fs::remove_file(&download_location).unwrap();
    }
}
