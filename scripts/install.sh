#!/usr/bin/env bash

set -euo pipefail

REPO="bokshi-gh/ghfetch"
INSTALL_DIR="${GHFETCH_INSTALL_DIR:-$HOME/.local/bin}"
VERSION="${GHFETCH_VERSION:-latest}"

if [[ "$VERSION" == "latest" ]]; then
    RELEASE_URL="https://github.com/$REPO/releases/latest/download"
else
    RELEASE_URL="https://github.com/$REPO/releases/download/$VERSION"
fi

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux)
        case "$ARCH" in
            x86_64)
                ASSET="ghfetch-linux-x86_64.tar.gz"
                ;;

            aarch64|arm64)
                ASSET="ghfetch-linux-aarch64.tar.gz"
                ;;

            *)
                echo "error: unsupported architecture: $ARCH"
                exit 1
                ;;
        esac
        ;;

    Darwin)
        case "$ARCH" in
            x86_64)
                ASSET="ghfetch-macos-x86_64.tar.gz"
                ;;

            arm64|aarch64)
                ASSET="ghfetch-macos-aarch64.tar.gz"
                ;;

            *)
                echo "error: unsupported architecture: $ARCH"
                exit 1
                ;;
        esac
        ;;

    *)
        echo "error: unsupported operating system: $OS"
        exit 1
        ;;
esac

TMP_DIR="$(mktemp -d)"

cleanup() {
    rm -rf "$TMP_DIR"
}

trap cleanup EXIT

ARCHIVE="$TMP_DIR/$ASSET"

echo "Installing ghfetch..."
echo "  OS:           $OS"
echo "  Architecture: $ARCH"
echo "  Version:      $VERSION"
echo

echo "Downloading $ASSET..."

curl \
    --fail \
    --location \
    --silent \
    --show-error \
    "$RELEASE_URL/$ASSET" \
    --output "$ARCHIVE"

echo "Extracting..."

tar \
    --extract \
    --gzip \
    --file "$ARCHIVE" \
    --directory "$TMP_DIR"

mkdir -p "$INSTALL_DIR"

install \
    "$TMP_DIR/ghfetch" \
    "$INSTALL_DIR/ghfetch"

echo
echo "ghfetch installed successfully."
echo
echo "Binary:"
echo "  $INSTALL_DIR/ghfetch"
echo

if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "Note: $INSTALL_DIR is not in your PATH."
    echo
    echo "Add it with:"
    echo
    echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo
fi
