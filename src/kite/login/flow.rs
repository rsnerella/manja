///! KiteConnect login flow.
///
///! See the official [KiteConnect API documentation.](https://kite.trade/docs/connect/v3/user/#login-flow).
use crate::kite::connect::credentials::KiteCredentials;
use crate::kite::error::{ManjaError, Result};
use crate::kite::login::{
    chrome::launch_browser, tokio_sleep, totp::generate_totp, BrowserClient, TokioDuration,
};
use fantoccini::Locator;
use url::Url;

pub async fn login_flow(
    base_url_login: &Url,
    redirect_url: &Url,
    kite_credentials: &KiteCredentials,
) -> Result<String> {
    // Launch the browser and WebDriver process
    let (client, mut driver) = launch_browser().await?;
    // Navigate to the Zerodha login page
    let _ = client
        .goto(&format!(
            "{}?api_key={}",
            base_url_login.as_str(),
            kite_credentials.api_key().as_str()
        ))
        .await;
    // Enter login ID
    client
        .wait()
        .for_element(Locator::XPath(r#"//*[@id="userid"]"#))
        .await?
        .send_keys(kite_credentials.user_id().as_str())
        .await?;
    // Enter password
    client
        .wait()
        .for_element(Locator::XPath(r#"//*[@id="password"]"#))
        .await?
        .send_keys(kite_credentials.user_pwd().as_str())
        .await?;
    // Click the login button
    client
        .wait()
        .for_element(Locator::XPath(
            r#"//*[@id="container"]/div/div/div[2]/form/div[4]/button"#,
        ))
        .await?
        .click()
        .await?;
    // Generate the TOTP code for the current time
    let current_code = generate_totp(kite_credentials.totp_key().as_str());
    // Enter TOTP access token
    client
        .wait()
        .for_element(Locator::XPath(r#"//*[@label="External TOTP"]"#))
        .await?
        .send_keys(&current_code)
        .await?;

    match wait_for_url(&client, redirect_url.clone(), TokioDuration::from_secs(10)).await {
        Ok(url_token) => {
            match url_token
                .query_pairs()
                .find(|(key, _)| key == "request_token")
                .map(|(_, value)| value.to_string())
            {
                Some(request_token) => {
                    client.close().await?;
                    driver.kill().await?;
                    Ok(request_token)
                }
                None => Err(ManjaError::from(
                    "`request_token` not found in redirect URL.",
                )),
            }
        }
        Err(e) => Err(e),
    }
}

pub async fn wait_for_url(
    client: &BrowserClient,
    url_base: Url,
    timeout: TokioDuration,
) -> Result<Url> {
    let start = tokio::time::Instant::now();
    while start.elapsed() < timeout {
        // Get the current URL
        match client.current_url().await {
            Ok(current_url) => {
                if current_url.domain() == url_base.domain() {
                    return Ok(current_url);
                }
            }
            Err(e) => return Err(ManjaError::from(e)),
        }
        // Wait for a short duration before checking again
        tokio_sleep(TokioDuration::from_millis(200)).await;
    }
    Err("Timed out waiting for redirect URL".into())
}
