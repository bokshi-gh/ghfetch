use anyhow::{Context, Result};
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT},
    Client,
};
use serde::Deserialize;

use crate::error::GitHubError;
use crate::parser::GitHubUrl;

const API_URL: &str = "https://api.github.com";

#[derive(Debug, Deserialize)]
pub struct Content {
    pub name: String,
    pub path: String,

    #[serde(rename = "download_url")]
    pub download_url: Option<String>,

    #[serde(rename = "type")]
    pub entry_type: EntryType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntryType {
    File,
    Dir,
    Symlink,
    Submodule,
}

pub struct GitHubClient {
    client: Client,
}

impl GitHubClient {
    pub fn new(token: Option<String>) -> Result<Self> {
        let mut headers = HeaderMap::new();

        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("ghfetch"),
        );

        headers.insert(
            ACCEPT,
            HeaderValue::from_static(
                "application/vnd.github+json",
            ),
        );

        if let Some(token) = token {
            let value = format!("Bearer {token}");

            let value = HeaderValue::from_str(&value)
                .context("invalid GitHub token")?;

            headers.insert(AUTHORIZATION, value);
        }

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .context("failed to create HTTP client")?;

        Ok(Self { client })
    }

    pub async fn get_contents(
        &self,
        github_url: &GitHubUrl,
    ) -> Result<Vec<Content>> {
        let url = format!(
            "{API_URL}/repos/{}/{}/contents/{}?ref={}",
            github_url.owner,
            github_url.repo,
            github_url.path,
            github_url.branch
        );

        let response = self
            .client
            .get(url)
            .send()
            .await
            .context("GitHub API request failed")?;

        let status = response.status();

        if !status.is_success() {
            return Err(self.handle_error(response).await?.into());
        }

        let value = response
            .json::<serde_json::Value>()
            .await
            .context("invalid GitHub API response")?;

        if value.is_array() {
            let entries = serde_json::from_value(value)
                .context("invalid directory response")?;

            Ok(entries)
        } else {
            let file = serde_json::from_value(value)
                .context("invalid file response")?;

            Ok(vec![file])
        }
    }

    pub async fn download(
        &self,
        url: &str,
    ) -> Result<Vec<u8>> {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .context("file download request failed")?;

        let status = response.status();

        if !status.is_success() {
            return Err(self.handle_error(response).await?.into());
        }

        let bytes = response
            .bytes()
            .await
            .context("failed to read downloaded file")?;

        Ok(bytes.to_vec())
    }

    async fn handle_error(
        &self,
        response: reqwest::Response,
    ) -> Result<GitHubError> {
        let status = response.status();

        let body = response
            .text()
            .await
            .unwrap_or_default();

        let error = match status.as_u16() {
            401 => GitHubError::Unauthorized,

            403 => {
                if body.contains("rate limit") {
                    GitHubError::RateLimited
                } else {
                    GitHubError::Forbidden
                }
            }

            404 => GitHubError::NotFound,

            _ => GitHubError::Api(format!(
                "HTTP {}: {}",
                status,
                body
            )),
        };

        Ok(error)
    }
}
