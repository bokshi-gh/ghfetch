mod cli;
mod downloader;
mod error;
mod github;
mod parser;

use anyhow::Result;
use clap::Parser;

use cli::Args;
use downloader::Downloader;
use github::GitHubClient;
use parser::GitHubUrl;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let github_url = GitHubUrl::parse(&args.url)?;

    let token = args.token.or_else(|| {
        std::env::var("GITHUB_TOKEN").ok()
    });

    let client = GitHubClient::new(token)?;

    let downloader = Downloader::new(client, args.output);

    downloader.fetch(github_url).await?;

    Ok(())
}
