use std::fs::File;
use std::io::Cursor;
use std::path::Path;

use reqwest::redirect::Policy;
use reqwest::Client;

/// reference: https://georgik.rocks/how-to-download-binary-file-in-rust-by-reqwest/
pub async fn download_file(url: &str, destination: impl AsRef<Path>) -> anyhow::Result<()> {
    let client = Client::builder().redirect(Policy::limited(5)).build()?;
    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to download file: {}", response.status());
    }

    let mut file = File::create(destination)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;

    Ok(())
}
