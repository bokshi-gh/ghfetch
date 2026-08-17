use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "ghfetch",
    version,
    about = "Fetch files and directories from GitHub"
)]
pub struct Args {
    /// GitHub URL
    pub url: String,

    /// Output path
    ///
    /// For repositories and directories:
    /// destination directory.
    ///
    /// For files:
    /// filename or existing directory.
    #[arg(short, long)]
    pub output: Option<String>,

    /// GitHub personal access token
    #[arg(short, long)]
    pub token: Option<String>,
}
