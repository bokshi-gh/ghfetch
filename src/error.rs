use std::fmt;

#[derive(Debug)]
pub enum GitHubError {
    Unauthorized,
    Forbidden,
    NotFound,
    RateLimited,
    Api(String),
}

impl fmt::Display for GitHubError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Unauthorized => {
                write!(f, "authentication required")
            }

            Self::Forbidden => {
                write!(f, "access forbidden")
            }

            Self::NotFound => {
                write!(
                    f,
                    "repository or resource not found"
                )
            }

            Self::RateLimited => {
                write!(
                    f,
                    "GitHub API rate limit exceeded"
                )
            }

            Self::Api(message) => {
                write!(
                    f,
                    "GitHub API error: {message}"
                )
            }
        }
    }
}

impl std::error::Error for GitHubError {}
