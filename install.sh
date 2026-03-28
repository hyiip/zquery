#!/usr/bin/env bash
set -euo pipefail

REPO="hyiip/zquery"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Detect platform
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS-$ARCH" in
  Linux-x86_64)  ARTIFACT="zquery-linux-x86_64" ;;
  Darwin-x86_64) ARTIFACT="zquery-macos-x86_64" ;;
  Darwin-arm64)  ARTIFACT="zquery-macos-arm64" ;;
  *) echo "Unsupported platform: $OS $ARCH"; exit 1 ;;
esac

# Get latest release tag via API
TAG=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | sed 's/.*: "\(.*\)".*/\1/')
if [ -z "$TAG" ]; then
  echo "Error: could not find latest release"
  exit 1
fi

echo "Installing zquery $TAG for $OS $ARCH..."

# Download and extract
WORK_DIR=$(mktemp -d)
trap 'rm -rf "$WORK_DIR"' EXIT
curl -sL "https://github.com/$REPO/releases/download/$TAG/$ARTIFACT.tar.gz" -o "$WORK_DIR/zquery.tar.gz"
tar xzf "$WORK_DIR/zquery.tar.gz" -C "$WORK_DIR"

# Install binary
mkdir -p "$INSTALL_DIR"
cp "$WORK_DIR/$ARTIFACT" "$INSTALL_DIR/zquery"
chmod +x "$INSTALL_DIR/zquery"
echo "Installed zquery to $INSTALL_DIR/zquery"

# Install Claude Code skill
SKILL_DIR="$HOME/.claude/skills/zquery"
mkdir -p "$SKILL_DIR"
cp "$WORK_DIR/skill/SKILL.md" "$SKILL_DIR/"
echo "Installed Claude Code skill to $SKILL_DIR"

# Check PATH
if ! echo "$PATH" | tr ':' '\n' | grep -qx "$INSTALL_DIR"; then
  echo ""
  echo "Note: $INSTALL_DIR is not in your PATH. Add it with:"
  echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
fi

echo "Done."
