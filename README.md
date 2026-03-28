# zquery

A Rust CLI for browsing your Zotero library. Designed as an interface for coding agents (like Claude) to discover and read collected literature.

Reads the local Zotero SQLite database directly, works while Zotero is running.

## Install

### Quick install (downloads binary + Claude Code skill)

```bash
# Linux/macOS
curl -sSL https://raw.githubusercontent.com/hyiip/zquery/main/install.sh | bash

# Windows (PowerShell)
irm https://raw.githubusercontent.com/hyiip/zquery/main/install.ps1 | iex
```

### From source

```bash
cargo install --path .

# Install the Claude Code skill manually
mkdir -p ~/.claude/skills/zquery            # Linux/macOS
cp skill/SKILL.md ~/.claude/skills/zquery/
```

The install scripts download the release binary to `~/.local/bin` and copy the [Claude Code skill](skill/SKILL.md) to `~/.claude/skills/zquery/`.

If `~/.local/bin` is not on your PATH, add it:

```bash
# Linux/macOS (add to ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.local/bin:$PATH"

# Windows (PowerShell, run once)
[Environment]::SetEnvironmentVariable('PATH', "$env:USERPROFILE\.local\bin;" + [Environment]::GetEnvironmentVariable('PATH', 'User'), 'User')
```

Any Claude Code session can then use `/zquery` to browse your library.

## Usage

### Browse collections

```bash
zquery ls                    # top-level collections
zquery ls "bert class"       # subcollections
zquery ls "bert class/voter" # items in a collection
```

### View detailed metadata

```bash
zquery show "bert class/voter"  # abstract, DOI, URL, journal, etc.
```

### Get PDF path

```bash
zquery pdf "bert class/voter" --item 1        # by index
zquery pdf "bert class/voter" --item "SPOOF"  # by title substring
```

### Export PDFs to local folder

Copies PDFs into a local directory so sandboxed agents can read them without needing access to the Zotero storage folder.

```bash
zquery export "bert class/voter"              # all PDFs -> ./zotero_pdfs/
zquery export "bert class/voter" --dest .     # copy to current directory
zquery export "bert class/voter" --item 1     # export one specific PDF
```

### Output formats

```bash
zquery ls "bert class/voter" --format json    # machine-readable for agents
zquery ls "bert class/voter" --format plain   # one line per item
zquery ls "bert class/voter" --format table   # default, human-readable
```

### Configuration

The Zotero data directory is auto-detected. Override with:

```bash
zquery --data-dir /path/to/Zotero ls
# or
export ZOTERO_DATA_DIR=/path/to/Zotero
```

## Example output

```
$ zquery ls "bert class/voter"
Items in: bert class/voter (3)
+---+----------------------------+------+----------------------------------+-----+
| # | Authors                    | Year | Title                            | PDF |
+===+===========================+======+==================================+=====+
| 1 | Jędrzejewski, Sznajd-Weron | 2019 | Statistical Physics Of Opinion.. | yes |
| 2 | Redner                     | 2019 | Reality-inspired voter models..  | yes |
| 3 | Fortunato et al.           | 2005 | Vector opinion dynamics in a..   | yes |
+---+----------------------------+------+----------------------------------+-----+
```

## How it works

Reads `zotero.sqlite` in immutable mode (`?mode=ro&immutable=1`) to avoid conflicts with a running Zotero instance. Collection paths are resolved by walking the parent-child hierarchy. Metadata is queried from Zotero's EAV schema (`itemData`/`itemDataValues`/`fields`). PDF paths are resolved from `itemAttachments` to absolute filesystem paths.
