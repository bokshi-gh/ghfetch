use anyhow::{Context, Result};
use reqwest::{
    Client,
    header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue, USER_AGENT},
};
use serde::Deserialize;

use crate::error::GitHubError;
use crate::parser::GitHubUrl;

const API_URL: &str = "https://api.github.com";

#[derive(Debug, Deserialize)]
pub struct Repository {
    pub default_branch: String,
}

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

        headers.insert(USER_AGENT, HeaderValue::from_static("ghfetch"));

        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/vnd.github+json"),
        );

        if let Some(token) = token {
            let value = format!("Bearer {token}");

            let value = HeaderValue::from_str(&value).context("invalid GitHub token")?;

            headers.insert(AUTHORIZATION, value);
        }

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .context("failed to create HTTP client")?;

        Ok(Self { client })
    }

    pub async fn get_default_branch(&self, owner: &str, repo: &str) -> Result<String> {
        let url = format!("{API_URL}/repos/{owner}/{repo}");

        let response = self
            .client
            .get(url)
            .send()
            .await
            .context("GitHub repository request failed")?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await?.into());
        }

        let repository = response
            .json::<Repository>()
            .await
            .context("invalid GitHub repository response")?;

        Ok(repository.default_branch)
    }

    pub async fn get_contents(&self, github_url: &GitHubUrl) -> Result<Vec<Content>> {
        let branch = github_url
            .branch
            .as_deref()
            .context("GitHub branch is missing")?;

        let path = if github_url.path.is_empty() {
            String::new()
        } else {
            format!("/{}", github_url.path)
        };

        let url = format!(
            "{API_URL}/repos/{}/{}/contents{}?ref={}",
            github_url.owner, github_url.repo, path, branch,
        );

        let response = self
            .client
            .get(url)
            .send()
            .await
            .context("GitHub API request failed")?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await?.into());
        }

        let value = response
            .json::<serde_json::Value>()
            .await
            .context("invalid GitHub API response")?;

        if value.is_array() {
            let entries = serde_json::from_value(value).context("invalid directory response")?;

            Ok(entries)
        } else {
            let file = serde_json::from_value(value).context("invalid file response")?;

            Ok(vec![file])
        }
    }

    pub async fn download(&self, url: &str) -> Result<Vec<u8>> {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .context("file download request failed")?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await?.into());
        }

        let bytes = response
            .bytes()
            .await
            .context("failed to read downloaded file")?;

        Ok(bytes.to_vec())
    }

    async fn handle_error(&self, response: reqwest::Response) -> Result<GitHubError> {
        let status = response.status();

        let body = response.text().await.unwrap_or_default();

        let error = match status.as_u16() {
            401 => GitHubError::Unauthorized,

            403 => {
                if body.to_lowercase().contains("rate limit") {
                    GitHubError::RateLimited
                } else {
                    GitHubError::Forbidden
                }
            }

            404 => GitHubError::NotFound,

            _ => GitHubError::Api(format!("HTTP {}: {}", status, body)),
        };

        Ok(error)
    }
}
