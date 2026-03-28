---
name: zquery
description: Browse and read papers from a Zotero library. Use when the user asks about their literature, references, papers, or when you need to find and read PDFs from Zotero.
allowed-tools: Bash(zquery *)
argument-hint: [collection-path]
---

# zquery — Zotero Library Browser

`zquery` is a standalone CLI binary installed on the user's PATH. Run it directly from any working directory — do NOT cd into the skill directory or look for scripts. Just run `zquery` as a shell command.

It reads the local Zotero SQLite database to browse collections, find papers, and read PDFs.

## Commands

### Browse collections and items

```bash
# List top-level collections
zquery ls

# List subcollections
zquery ls "bert class"

# List items in a collection (shows author, year, title)
zquery ls "daily_read/voter"
```

### Get detailed metadata

```bash
# Full metadata: abstract, DOI, URL, journal, volume, pages
zquery show "bert class/voter"
```

### Get PDF file path

```bash
# By index (from ls output)
zquery pdf "bert class/voter" --item 1

# By title substring (case-insensitive)
zquery pdf "bert class/voter" --item "SPOOF"
```

### Export PDFs to a local folder

Use this when you need to read a PDF but don't have access to the Zotero storage directory.

```bash
# Export all PDFs in a collection to ./zotero_pdfs/
zquery export "bert class/voter"

# Export to current directory
zquery export "bert class/voter" --dest .

# Export one specific PDF
zquery export "bert class/voter" --item 1
```

## Output formats

Always use `--format json` when you need to parse the output programmatically.

```bash
# JSON (for parsing)
zquery ls "bert class/voter" --format json

# Plain text (one line per item: "Author (Year) - Title")
zquery ls "bert class/voter" --format plain

# Table (default, human-readable)
zquery ls "bert class/voter" --format table
```

## Typical workflow for reading a paper

```bash
# 1. Find the collection
zquery ls

# 2. Browse into it
zquery ls "bert class/voter"

# 3. Export the PDF you need to a local folder
zquery export "bert class/voter" --item 1 --dest .

# 4. Read the exported PDF
# (the export command prints the file path)
```

## Notes

- Collection paths use `/` as separator: `"parent/child/grandchild"`
- Path matching is case-insensitive
- `--item` accepts either a 1-based index or a title substring
- If a title substring matches multiple items, zquery lists them so you can be more specific
- If a collection path is wrong, zquery lists available collections at that level
