use std::fs::File;
use std::io::Cursor;
use std::path::Path;

use lazy_static::lazy_static;
use reqwest::redirect::Policy;
use reqwest::Client;

lazy_static! {
    static ref CLIENT: Client = Client::builder()
        .redirect(Policy::limited(5))
        .use_rustls_tls()
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ))
        .build()
        .unwrap_or_else(|err| panic!("Failed to create HTTP client: {}", err));
}

/// reference: https://georgik.rocks/how-to-download-binary-file-in-rust-by-reqwest/
pub async fn download_file(url: &str, destination: impl AsRef<Path>) -> anyhow::Result<()> {
    let response = CLIENT.get(url).send().await?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to download file: {}", response.status());
    }

    let mut file = File::create(destination)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;

    Ok(())
}
