use crate::kite::error::Result;
use crate::kite::login::BrowserClient;
use serde_json::Map;
use std::env;

type WebDriverProcess = tokio::process::Child;

pub async fn launch_browser() -> Result<(BrowserClient, WebDriverProcess)> {
    let chrome_binary_path = env::var("CHROME_BINARY_PATH")?;
    let chromedriver_path = env::var("CHROMEDRIVER_PATH")?;
    let port = 9515;

    // Start chromedriver on a specific port
    let driver = tokio::process::Command::new(chromedriver_path)
        .arg(format!("--port={}", port))
        .spawn()
        .expect("Failed to start chromedriver");

    // Allow some time for chromedriver to start up
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Set up capabilities specific to Chrome
    let mut caps = Map::new();
    caps.insert(
        "goog:chromeOptions".to_string(),
        serde_json::json!({
            "binary": chrome_binary_path,
            "args": ["--disable-gpu", "--window-size=1280,800"]
            // "args": ["--headless", "--disable-gpu", "--window-size=1280,800"]
        }),
    );

    let driver_url = format!("http://localhost:{}", port);

    // Connect to the WebDriver
    match fantoccini::ClientBuilder::native()
        .capabilities(caps)
        .connect(&driver_url)
        .await
    {
        Ok(client) => Ok((client, driver)),
        Err(e) => Err(e.into()),
    }
}
