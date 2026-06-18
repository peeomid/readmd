# readmd Build Spec

## Summary

Build `readmd`, a Rust CLI that converts one Markdown file into one beautiful standalone HTML reading page.

## Terms/context

- CLI: command line app.
- Standalone HTML: one `.html` file with CSS inside it.
- Theme: color preset.
- Style: typography and reader width preset.
- Config: TOML file that can choose the default theme/style and define custom themes.

## Overview

- Scope:
  - Accept `.md`, `.markdown`, and `.mdx`.
  - Render Markdown to sanitized HTML.
  - Wrap rendered content in a full HTML document.
  - Generate inline CSS from a selected theme and style.
  - Write to default path, chosen path, temp path, or stdout.
  - Print theme and config data for humans and agents.

- Not in scope:
  - Blog engine.
  - Static site generator.
  - Markdown editor.
  - PDF output.
  - Syntax highlighting engine.
  - Asset copying or image embedding.

## Commands

- `readmd <input.md>`
  - Writes beside input with `.html` extension.

- `readmd <input.md> --output <output.html>`
  - Writes to chosen output path.

- `readmd <input.md> --theme <name>`
  - Uses selected color theme.

- `readmd <input.md> --style <name>`
  - Uses selected typography and width style.

- `readmd <input.md> --config <readmd.toml>`
  - Loads explicit config.

- `readmd <input.md> --stdout`
  - Prints full HTML to stdout and writes no file.

- `readmd <input.md> --tmp`
  - Writes to a generated file in the system temp directory.

- `readmd <input.md> --open`
  - Opens the written file after successful write.

- `readmd themes list`
  - Prints available color theme names.

- `readmd themes print <name>`
  - Prints one theme as TOML.

- `readmd styles list`
  - Prints available reader style names.

- `readmd styles print <name>`
  - Prints one reader style as TOML.

- `readmd config print-default`
  - Prints the built-in default config as TOML.

## Flag Rules

- `--stdout` cannot combine with `--open`.
- `--stdout` cannot combine with `--output`.
- `--stdout` cannot combine with `--tmp`.
- `--output` cannot combine with `--tmp`.
- `--open` opens only a written file.

## Config Rules

- Lookup order:
  - `--config <path>`
  - `./readmd.toml`
  - `~/.config/readmd/config.toml`
  - built-in defaults

- Merge rule:
  - Built-in config loads first.
  - User config can change `default_theme`.
  - User config can change `default_style`.
  - User config can replace or add themes by name.

- Failure rule:
  - Explicit `--config` path missing is an error.
  - Automatic config paths missing are ignored.
  - Invalid TOML is an error.
  - Unknown theme/style combination is an error.

## Renderer Rules

- Extract behavior from CalmPage:
  - `.md`, `.markdown`, `.mdx` detection.
  - Frontmatter split.
  - Frontmatter metadata block.
  - Markdown render with `pulldown-cmark`.
  - Sanitized output with `ammonia`.
  - Title from frontmatter `title`, first `# heading`, or file name.

- Markdown features:
  - Tables.
  - Footnotes.
  - Strikethrough.
  - Task lists.
  - Heading attributes.

## HTML Rules

- Output must include:
  - `<!doctype html>`
  - `<html lang="en">`
  - `<meta charset="utf-8">`
  - responsive viewport meta
  - escaped `<title>`
  - inline `<style>`
  - `<main class="page"><article class="reader">...</article></main>`

## Test Spec

- Unit tests:
  - Renderer sanitizes script HTML.
  - Renderer turns frontmatter into metadata block.
  - Markdown extension detection includes MDX.
  - Title picks frontmatter title first.
  - Theme CSS contains key class rules.
  - Config merge can add a custom theme.

- CLI tests:
  - Default output writes beside input.
  - `--output` writes chosen path.
  - `--stdout` prints HTML and writes no default file.
  - `--tmp` writes temp HTML and prints the path.
  - Invalid flag combos fail.
  - `themes list` includes built-ins.
  - `styles list` includes built-ins.
  - `themes print paper` prints TOML.
  - `config print-default` prints TOML.

## Done Definition

- `cargo fmt` passes.
- `cargo test` passes.
- CLI can render `README.md` into a standalone HTML file.
- Help text includes purpose, examples, output behavior, and config lookup order.
