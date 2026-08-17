use anyhow::{bail, Context, Result};
use url::Url;

#[derive(Debug, Clone, Copy)]
pub enum GitHubResource {
    File,
    Directory,
}

#[derive(Debug, Clone)]
pub struct GitHubUrl {
    pub owner: String,
    pub repo: String,
    pub branch: String,
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

        if parts.len() < 4 {
            bail!(
                "expected a URL like \
                 https://github.com/owner/repo/tree/branch/path"
            );
        }

        let owner = parts[0].to_string();

        let repo = parts[1]
            .strip_suffix(".git")
            .unwrap_or(parts[1])
            .to_string();

        let resource = match parts[2] {
            "blob" => GitHubResource::File,
            "tree" => GitHubResource::Directory,
            other => {
                bail!(
                    "unsupported GitHub resource '{}'; \
                     expected 'blob' or 'tree'",
                    other
                )
            }
        };

        /*
         * GitHub branches may contain '/'.
         *
         * We cannot blindly assume parts[3] is the complete branch.
         *
         * Example:
         *
         * /tree/feature/my-branch/src
         *
         * becomes:
         *
         * feature/my-branch
         * src
         *
         * We determine the split later through the GitHub API.
         *
         * For now, the common case is handled here.
         */

        let branch = parts[3].to_string();
        let path = parts[4..].join("/");

        if path.is_empty() {
            bail!("missing file or directory path");
        }

        Ok(Self {
            owner,
            repo,
            branch,
            path,
            resource,
        })
    }

    pub fn api_url(&self) -> String {
        format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            self.owner,
            self.repo,
            self.path
        )
    }
}
