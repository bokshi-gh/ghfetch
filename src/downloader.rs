use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use async_recursion::async_recursion;
use tokio::fs;

use crate::{
    github::{Content, EntryType, GitHubClient},
    parser::{GitHubResource, GitHubUrl},
};

pub struct Downloader {
    client: GitHubClient,
    output: Option<PathBuf>,
}

impl Downloader {
    pub fn new(client: GitHubClient, output: Option<String>) -> Self {
        Self {
            client,
            output: output.map(PathBuf::from),
        }
    }

    pub async fn fetch(&self, mut github_url: GitHubUrl) -> Result<()> {
        match github_url.resource {
            GitHubResource::Repository => {
                let branch = self
                    .client
                    .get_default_branch(&github_url.owner, &github_url.repo)
                    .await?;

                github_url.branch = Some(branch);

                let destination = self.directory_destination(&github_url.repo);

                fs::create_dir_all(&destination)
                    .await
                    .context("failed to create output directory")?;

                println!("fetch repository → {}", destination.display());

                self.fetch_directory(&github_url, "", &destination).await?;
            }

            GitHubResource::Directory => {
                let directory_name = Path::new(&github_url.path)
                    .file_name()
                    .and_then(|name| name.to_str())
                    .context("could not determine directory name")?;

                let destination = self.directory_destination(directory_name);

                fs::create_dir_all(&destination)
                    .await
                    .context("failed to create output directory")?;

                println!("fetch directory → {}", destination.display());

                self.fetch_directory(&github_url, "", &destination).await?;
            }

            GitHubResource::File => {
                let entries = self.client.get_contents(&github_url).await?;

                let file = entries.into_iter().next().context("file not found")?;

                let destination = self.file_destination(&file)?;

                self.download_file(&file, &destination).await?;
            }
        }

        Ok(())
    }

    fn directory_destination(&self, default_name: &str) -> PathBuf {
        match &self.output {
            Some(output) => output.clone(),
            None => PathBuf::from(default_name),
        }
    }

    fn file_destination(&self, file: &Content) -> Result<PathBuf> {
        let file_name = Path::new(&file.name);

        match &self.output {
            None => Ok(file_name.to_path_buf()),

            Some(output) => {
                if output.is_dir() {
                    Ok(output.join(file_name))
                } else {
                    Ok(output.clone())
                }
            }
        }
    }

    #[async_recursion]
    async fn fetch_directory(
        &self,
        github_url: &GitHubUrl,
        relative_path: &str,
        destination: &Path,
    ) -> Result<()> {
        let mut url = github_url.clone();

        url.path = if relative_path.is_empty() {
            github_url.path.clone()
        } else if github_url.path.is_empty() {
            relative_path.to_string()
        } else {
            format!("{}/{}", github_url.path, relative_path)
        };

        let entries = self.client.get_contents(&url).await?;

        for entry in entries {
            let relative = if relative_path.is_empty() {
                entry.name.clone()
            } else {
                format!("{}/{}", relative_path, entry.name)
            };

            match entry.entry_type {
                EntryType::File => {
                    let file_path = destination.join(&relative);

                    self.download_file(&entry, &file_path).await?;
                }

                EntryType::Dir => {
                    self.fetch_directory(github_url, &relative, destination)
                        .await?;
                }

                EntryType::Symlink => {
                    eprintln!("skip symlink: {}", entry.path);
                }

                EntryType::Submodule => {
                    eprintln!("skip submodule: {}", entry.path);
                }
            }
        }

        Ok(())
    }

    async fn download_file(&self, entry: &Content, destination: &Path) -> Result<()> {
        let download_url = entry
            .download_url
            .as_deref()
            .context("GitHub did not provide a download URL")?;

        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)
                .await
                .context("failed to create directory")?;
        }

        println!("fetch {}", destination.display());

        let data = self.client.download(download_url).await?;

        fs::write(destination, data)
            .await
            .with_context(|| format!("failed to write {}", destination.display()))?;

        Ok(())
    }
}
