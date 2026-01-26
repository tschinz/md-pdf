<div align="center">
  <img src="img/md-pdf.svg" alt="md-pdf logo" width="150">
</div>

# md-pdf

A fast, lightweight command-line tool that converts Markdown files to professional PDF documents using [Typst](https://typst.app).

## Features

- 🚀 **Fast conversion** powered by Typst
- 🎨 **Professional output** with built-in templates
- ⚙️ **Zero configuration** - works out of the box
- 👀 **Watch mode** for live preview
- 🔗 **Link validation** checks external URLs
- 📝 **Rich metadata** support via YAML front matter

## Installation

### Prerequisites

### Typst

Install [Typst CLI](https://github.com/typst/typst):

```bash
# macOS
brew install typst

# Or download from GitHub releases
# https://github.com/typst/typst/releases
```

### Fonts

The default template `simple` uses some fonts which are optional.

Install [Iosevka](https://typeof.net/Iosevka/)

### Install md-pdf

```bash
# From source
cargo install --git https://github.com/tschinz/md-pdf

# Or build locally
git clone https://github.com/tschinz/md-pdf
cd md-pdf && cargo install --path .
```

## Quick Start

```bash
# Convert markdown to PDF
md-pdf document.md

# Watch for changes (live preview)
md-pdf --watch document.md

# Check links before conversion
md-pdf --check-links document.md

# Use specific template
md-pdf document.md -t none
md-pdf document.md -t simple
md-pdf document.md -t playful
md-pdf document.md -t brutalist
```

## Usage

```
Convert markdown files to PDF using typst with templating

Usage: md-pdf [OPTIONS] [INPUT]

Arguments:
  [INPUT]  Path to the input Markdown file

Options:
  -o, --output <OUTPUT>      Path to the output PDF file
  -t, --template <TEMPLATE>  list the templates and select the one you want [default: none]
  -w, --watch                Watch the input file for changes and rebuild automatically
      --check-links          Check all links in the markdown file and display warnings for unreachable links
      --list-templates       List all available templates
      --create-config        Create default configuration file
      --show-config          Show configuration file locations and settings
  -h, --help                 Print help
  -V, --version              Print version
```

## Templates

- `none` - Minimal styling (default)
- `simple` - Professional with headers/footers
- `playful` - colorful inspired by Dieter Rams
- `brutalist` - Raw, bold, stark design with high contrast
- You can add you own templates

![](examples/resources/starfleet-none.png)
![](examples/resources/starfleet-simple.png)
![](examples/resources/starfleet-playful.png)
![](examples/resources/starfleet-brutalist.png)

## Front Matter

Add metadata to your markdown:

```yaml
---
title: "My Document"
author: "Your Name"
date: "2026-01-23"
template: "simple"
toc: true
---
# Content starts here
```

## Configuration

Auto-created at `~/.config/md-pdf/config.ron` on first run. Customize defaults:

```rust
(
    templates_dir: Some("/Users/username/.config/md-pdf/templates"),
    default_template: Some("simple"),
    default_author: Some("Your Name"),
    default_language: Some("en"),
    default_toc: Some(true),
)
```

## Examples

See [`examples/comprehensive-guide.md`](examples/comprehensive-guide.md) for full documentation and feature demonstrations.

```
md-pdf example/comprehensive-guide.md
```
