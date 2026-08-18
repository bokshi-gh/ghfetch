# ghfetch

Fetch files, directories, and repositories directly from GitHub.

`ghfetch` is a Rust CLI that uses the GitHub API to recursively download repository resources while preserving their directory structure.

## Features

- Fetch an entire GitHub repository
- Fetch a specific directory recursively
- Fetch a single file
- Preserve directory structure
- Custom output paths with `-o` / `--output`
- GitHub token authentication for private repositories
- Cross-platform prebuilt binaries
- Shell and PowerShell installers
- No Git installation required

## Installation

### Linux / macOS

Install the latest prebuilt release:

```bash
curl -fsSL \
  https://raw.githubusercontent.com/bokshi-gh/ghfetch/main/scripts/install.sh \
  | sh
````

The installer detects your operating system and architecture, downloads the appropriate GitHub Release binary, and installs it to:

```text
~/.local/bin/ghfetch
```

You can customize the installation directory:

```bash
GHFETCH_INSTALL_DIR="$HOME/bin" \
curl -fsSL \
  https://raw.githubusercontent.com/bokshi-gh/ghfetch/main/scripts/install.sh \
  | sh
```

### Windows

Run PowerShell:

```powershell
irm https://raw.githubusercontent.com/bokshi-gh/ghfetch/main/scripts/install.ps1 | iex
```

The binary is installed to:

```text
%USERPROFILE%\.ghfetch\bin
```

### Cargo

If you have Rust and Cargo installed:

```bash
cargo install ghfetch
```

### From source

Clone the repository:

```bash
git clone https://github.com/bokshi-gh/ghfetch.git
cd ghfetch
```

Build:

```bash
cargo build --release
```

The binary will be available at:

```text
target/release/ghfetch
```

## Usage

```text
ghfetch [OPTIONS] <URL>
```

### Options

```text
-o, --output <OUTPUT>   Output path
-t, --token <TOKEN>     GitHub personal access token
-h, --help              Print help
-V, --version           Print version
```

## Examples

### Fetch a repository

```bash
ghfetch https://github.com/bokshi-gh/ghfetch
```

The repository is downloaded into:

```text
ghfetch/
```

For example:

```text
ghfetch/
├── Cargo.toml
├── README.md
└── src/
    ├── cli.rs
    ├── downloader.rs
    ├── error.rs
    ├── github.rs
    ├── main.rs
    └── parser.rs
```

### Fetch a repository to a custom directory

```bash
ghfetch \
  https://github.com/bokshi-gh/ghfetch \
  -o ./download
```

Result:

```text
download/
├── Cargo.toml
├── README.md
└── src/
    └── ...
```

### Fetch a directory

```bash
ghfetch \
  https://github.com/bokshi-gh/ghfetch/tree/main/src
```

Without `-o`, the directory name is used:

```text
src/
├── cli.rs
├── downloader.rs
├── error.rs
├── github.rs
├── main.rs
└── parser.rs
```

### Fetch a directory to a custom location

```bash
ghfetch \
  https://github.com/bokshi-gh/ghfetch/tree/main/src \
  -o ./my-src
```

Result:

```text
my-src/
├── cli.rs
├── downloader.rs
├── error.rs
├── github.rs
├── main.rs
└── parser.rs
```

### Fetch a single file

```bash
ghfetch \
  https://github.com/bokshi-gh/ghfetch/blob/main/src/main.rs
```

The file is downloaded as:

```text
main.rs
```

### Rename a downloaded file

Use `-o` with a filename:

```bash
ghfetch \
  https://github.com/bokshi-gh/ghfetch/blob/main/src/main.rs \
  -o hello.rs
```

Result:

```text
hello.rs
```

### Download a file into an existing directory

```bash
mkdir downloads

ghfetch \
  https://github.com/bokshi-gh/ghfetch/blob/main/src/main.rs \
  -o downloads
```

Result:

```text
downloads/
└── main.rs
```

## Private Repositories

`ghfetch` supports GitHub personal access tokens.

### Environment Variable

The recommended method is to use `GITHUB_TOKEN`:

```bash
export GITHUB_TOKEN="your_token"
```

Then:

```bash
ghfetch https://github.com/OWNER/PRIVATE_REPOSITORY
```

The token is automatically used for GitHub API requests.

### Command-Line Option

You can also provide a token directly:

```bash
ghfetch \
  https://github.com/OWNER/PRIVATE_REPOSITORY \
  --token "$GITHUB_TOKEN"
```

Using the environment variable is recommended because it avoids putting the token directly into your shell command.

## Authentication

For public repositories, authentication is optional.

```text
Public repository
      │
      ├── no token → works
      └── token    → works

Private repository
      │
      ├── no token → fails
      └── token    → works
```

The token is sent to GitHub through the HTTP `Authorization` header and is not included in the GitHub URL.

## Supported GitHub URLs

### Repository

```text
https://github.com/OWNER/REPOSITORY
```

### Directory

```text
https://github.com/OWNER/REPOSITORY/tree/BRANCH/PATH
```

### File

```text
https://github.com/OWNER/REPOSITORY/blob/BRANCH/PATH
```

For example:

```text
https://github.com/bokshi-gh/ghfetch
```

```text
https://github.com/bokshi-gh/ghfetch/tree/main/src
```

```text
https://github.com/bokshi-gh/ghfetch/blob/main/src/main.rs
```

### Current Limitation

Branch names containing `/` are not currently supported in `tree` and `blob` URLs.

For example:

```text
https://github.com/OWNER/REPOSITORY/tree/feature/my-branch/src
```

is not currently handled correctly because `ghfetch` treats:

```text
feature
```

as the branch.

Simple branch names such as:

```text
main
develop
dev
release
v1
```

are supported.

## How It Works

`ghfetch` does not use `git clone`.

Instead, it communicates with GitHub's REST API:

```text
GitHub URL
    │
    ▼
URL Parser
    │
    ├── Repository
    ├── Directory
    └── File
    │
    ▼
GitHub API
    │
    ▼
Content information
    │
    ├── File
    │    └── Download
    │
    └── Directory
         └── Recursively fetch contents
              │
              ├── File → Download
              │
              └── Directory → Recurse
```

For a directory:

```text
src/
├── main.rs
├── cli.rs
└── utils/
    ├── mod.rs
    └── config.rs
```

`ghfetch` recursively requests:

```text
src/
src/main.rs
src/cli.rs
src/utils/
src/utils/mod.rs
src/utils/config.rs
```

and recreates the same structure locally.

## Why Not `git clone`?

`ghfetch` is designed for situations where you only want GitHub resources rather than a complete Git working copy.

For example:

```bash
ghfetch https://github.com/user/repo/tree/main/examples
```

downloads the `examples` directory without requiring a Git repository.

This means you don't need:

```text
.git/
branches
commits
Git history
```

Only the requested resources are downloaded.

## Project Structure

```text
ghfetch/
├── .github/
│   └── workflows/
│       └── release.yml
│
├── scripts/
│   ├── install.sh
│   └── install.ps1
│
├── src/
│   ├── cli.rs
│   ├── downloader.rs
│   ├── error.rs
│   ├── github.rs
│   ├── main.rs
│   └── parser.rs
│
├── .gitignore
├── Cargo.toml
├── Cargo.lock
├── LICENSE
└── README.md
```

## Development

Clone the repository:

```bash
git clone https://github.com/bokshi-gh/ghfetch.git
cd ghfetch
```

Build:

```bash
cargo build
```

Run:

```bash
cargo run -- https://github.com/bokshi-gh/ghfetch
```

Check:

```bash
cargo check
```

Format:

```bash
cargo fmt
```

Lint:

```bash
cargo clippy
```

Test:

```bash
cargo test
```

Release build:

```bash
cargo build --release
```

## Releases

Releases are built automatically through GitHub Actions when a version tag is pushed.

Create a tag:

```bash
git tag v0.1.0
```

Push it:

```bash
git push origin v0.1.0
```

The release workflow builds platform-specific binaries and publishes them to GitHub Releases.

Current release targets:

```text
Linux
├── x86_64
└── aarch64

macOS
├── x86_64
└── aarch64

Windows
├── x86_64
└── aarch64
```

## Security

Do not commit GitHub tokens to the repository.

Avoid:

```bash
ghfetch URL --token ghp_your_token
```

when possible, because command-line arguments can potentially be visible to other processes or stored in shell history.

Prefer:

```bash
export GITHUB_TOKEN="your_token"
```

then:

```bash
ghfetch URL
```

The token should have only the permissions required for the repositories you need to access.

## Requirements

### Building from Source

* Rust
* Cargo
* Internet connection

### Installing a Release

No Rust installation is required.

The installer downloads a prebuilt binary.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE).

## Status

`ghfetch` is currently under development.

The API, CLI interface, and project structure may change before the first stable release.
