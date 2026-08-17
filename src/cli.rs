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

    /// Output directory
    #[arg(short, long, default_value = ".")]
    pub output: String,

    /// GitHub personal access token
    #[arg(short, long)]
    pub token: Option<String>,
}
