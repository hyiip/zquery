# zquery

A Rust CLI for browsing your Zotero library. Designed as an interface for coding agents (like Claude) to discover and read collected literature.

Reads the local Zotero SQLite database directly — no network, no API key, works while Zotero is running.

## Install

```bash
cargo install --path .
```

### Claude Code skill

zquery ships with a [Claude Code skill](skill/SKILL.md) so coding agents know how to use it. To install:

```bash
# Linux/macOS
cp -r skill ~/.claude/skills/zquery

# Windows (PowerShell)
Copy-Item -Recurse skill "$env:USERPROFILE\.claude\skills\zquery"
```

Then any Claude Code session can use `/zquery` to learn how to browse your library.

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
