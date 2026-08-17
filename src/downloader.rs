use std::path::PathBuf;

use anyhow::{Context, Result};
use tokio::fs;

use crate::{
    github::{Content, EntryType, GitHubClient},
    parser::{GitHubResource, GitHubUrl},
};

pub struct Downloader {
    client: GitHubClient,
    output: PathBuf,
}

impl Downloader {
    pub fn new(
        client: GitHubClient,
        output: String,
    ) -> Self {
        Self {
            client,
            output: PathBuf::from(output),
        }
    }

    pub async fn fetch(
        &self,
        github_url: GitHubUrl,
    ) -> Result<()> {
        match github_url.resource {
            GitHubResource::File => {
                let entries = self
                    .client
                    .get_contents(&github_url)
                    .await?;

                let file = entries
                    .into_iter()
                    .next()
                    .context("file not found")?;

                self.download_file(
                    &file,
                    &file.name,
                )
                .await?;
            }

            GitHubResource::Directory => {
                self.fetch_directory(
                    &github_url,
                    "",
                )
                .await?;
            }
        }

        Ok(())
    }

    async fn fetch_directory(
        &self,
        github_url: &GitHubUrl,
        relative_path: &str,
    ) -> Result<()> {
        let mut url = github_url.clone();

        url.path = if relative_path.is_empty() {
            github_url.path.clone()
        } else {
            format!(
                "{}/{}",
                github_url.path,
                relative_path
            )
        };

        let entries = self
            .client
            .get_contents(&url)
            .await?;

        for entry in entries {
            let relative = if relative_path.is_empty() {
                entry.name.clone()
            } else {
                format!(
                    "{}/{}",
                    relative_path,
                    entry.name
                )
            };

            match entry.entry_type {
                EntryType::File => {
                    self.download_file(
                        &entry,
                        &relative,
                    )
                    .await?;
                }

                EntryType::Dir => {
                    self.fetch_directory(
                        github_url,
                        &relative,
                    )
                    .await?;
                }

                EntryType::Symlink => {
                    eprintln!(
                        "skip symlink: {}",
                        entry.path
                    );
                }

                EntryType::Submodule => {
                    eprintln!(
                        "skip submodule: {}",
                        entry.path
                    );
                }
            }
        }

        Ok(())
    }

    async fn download_file(
        &self,
        entry: &Content,
        relative_path: &str,
    ) -> Result<()> {
        let url = entry
            .download_url
            .as_deref()
            .context(
                "GitHub did not provide a download URL",
            )?;

        let destination =
            self.output.join(relative_path);

        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)
                .await
                .context(
                    "failed to create directory",
                )?;
        }

        println!(
            "fetch {}",
            destination.display()
        );

        let data = self
            .client
            .download(url)
            .await?;

        fs::write(&destination, data)
            .await
            .with_context(|| {
                format!(
                    "failed to write {}",
                    destination.display()
                )
            })?;

        Ok(())
    }
}
