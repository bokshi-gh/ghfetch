use anyhow::{bail, Context, Result};
use url::Url;

#[derive(Debug, Clone, Copy)]
pub enum GitHubResource {
    Repository,
    File,
    Directory,
}

#[derive(Debug, Clone)]
pub struct GitHubUrl {
    pub owner: String,
    pub repo: String,
    pub branch: Option<String>,
    pub path: String,
    pub resource: GitHubResource,
}

impl GitHubUrl {
    pub fn parse(input: &str) -> Result<Self> {
        let url = Url::parse(input)
            .with_context(|| format!("invalid URL: {input}"))?;

        if url.scheme() != "https" {
            bail!("URL must use HTTPS");
        }

        if url.host_str() != Some("github.com") {
            bail!("URL must be from github.com");
        }

        let parts: Vec<&str> = url
            .path_segments()
            .context("invalid GitHub URL")?
            .filter(|part| !part.is_empty())
            .collect();

        if parts.len() < 2 {
            bail!("invalid GitHub URL");
        }

        let owner = parts[0].to_string();

        let repo = parts[1]
            .strip_suffix(".git")
            .unwrap_or(parts[1])
            .to_string();

        // https://github.com/owner/repo
        if parts.len() == 2 {
            return Ok(Self {
                owner,
                repo,
                branch: None,
                path: String::new(),
                resource: GitHubResource::Repository,
            });
        }

        let resource = match parts[2] {
            "blob" => GitHubResource::File,
            "tree" => GitHubResource::Directory,
            other => {
                bail!(
                    "unsupported GitHub URL type '{}'; \
                     expected 'blob' or 'tree'",
                    other
                );
            }
        };

        if parts.len() < 5 {
            bail!("missing branch or path");
        }

        let branch = parts[3].to_string();
        let path = parts[4..].join("/");

        if path.is_empty() {
            bail!("missing file or directory path");
        }

        Ok(Self {
            owner,
            repo,
            branch: Some(branch),
            path,
            resource,
        })
    }
}
