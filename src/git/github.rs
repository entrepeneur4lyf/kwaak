use anyhow::{Context, Result};
use secrecy::{ExposeSecret, SecretString};

/// Adds the github token to the repository url
#[tracing::instrument(skip_all)]
pub fn add_token_to_url(repo_url: &str, token: &SecretString) -> Result<SecretString> {
    if !repo_url.starts_with("https://") {
        anyhow::bail!("Only https urls are supported")
    }

    let mut parsed = url::Url::parse(repo_url).context("Failed to parse url")?;

    parsed
        .set_username("x-access-token")
        .and_then(|()| parsed.set_password(Some(token.expose_secret())))
        .expect("Infallible");

    Ok(SecretString::from(parsed.to_string()))
}
