# readmd CLI Plan

## Summary

Build `readmd`: a small CLI that converts Markdown into beautiful standalone HTML pages.

## Terms/context

- Standalone HTML: one complete `.html` file that can open in browser.
- Theme: a named visual config set.
- Built-in theme: theme compiled into the CLI.
- User theme: theme loaded from `readmd.toml`.

## Requirements

- Input:
  - Accept one Markdown file path.
  - Support `.md`, `.markdown`, and `.mdx`.

- Output:
  - Default output path is input path with `.html`.
  - `--output <path>` writes to a chosen path.
  - `--tmp` writes the generated HTML to the system temp folder.
  - `--stdout` prints HTML to terminal.
  - `--open` opens the generated HTML right after writing it.
  - Full standalone HTML by default.
  - `--stdout` cannot be combined with `--open`, because there is no file to open.

- Config:
  - `--config <path>` loads a config file.
  - Fall back to `./readmd.toml`.
  - Fall back to `~/.config/readmd/config.toml`.
  - Fall back to built-in defaults.
  - Config can define multiple themes.
  - Config chooses `default_theme`.

- Themes:
  - Ship several built-in themes.
  - Allow user themes to override or add themes.
  - Theme config must cover typography and visual rendering.
  - Theme config should avoid Markdown parser settings unless needed later.

- Help:
  - Help must explain what the tool does.
  - Help must include examples.
  - Help must include output behavior.
  - Help must include config lookup order.
  - Help must be stable and clear for agents.

## Command Examples

Generate beside the Markdown file:

```bash
readmd file.md
```

Generate to a chosen path:

```bash
readmd file.md --output public/file.html
```

Generate to temp folder:

```bash
readmd file.md --tmp
```

Generate to temp folder and open in browser:

```bash
readmd file.md --tmp --open
```

Generate beside the Markdown file and open:

```bash
readmd file.md --open
```

Print HTML to terminal:

```bash
readmd file.md --stdout
```

Output path rules:

- `--output <path>` wins over default output path.
- `--tmp` writes to a generated file under the system temp folder.
- `--output` and `--tmp` should not be used together.
- `--open` opens the final written file.
- `--stdout` should not write a file.
- `--stdout --open` is invalid.

## CalmPage Source

`readmd` should reuse the existing CalmPage Markdown renderer instead of starting from scratch.

Source repo:

```text
/Users/luannguyenthanh/Development/Osimify/minimal-markdown-reader
```

Primary source file:

```text
/Users/luannguyenthanh/Development/Osimify/minimal-markdown-reader/src-tauri/src/lib.rs
```

Functions to extract:

- `is_markdown`
- `title_from_path`
- `split_frontmatter`
- `render_frontmatter`
- `render_markdown`
- `note_title`

Renderer behavior already available:

- Parses Markdown with `pulldown-cmark`.
- Supports tables.
- Supports footnotes.
- Supports strikethrough.
- Supports task lists.
- Supports heading attributes.
- Parses YAML frontmatter.
- Renders frontmatter as a document details block.
- Sanitizes HTML with `ammonia`.
- Escapes frontmatter values with `html-escape`.

Dependencies to bring over first:

```toml
pulldown-cmark = { version = "0.13", default-features = false, features = ["html"] }
serde = { version = "1", features = ["derive"] }
serde_yaml = "0.9"
html-escape = "0.2"
ammonia = "4"
```

Useful existing tests:

- `renderer_sanitizes_raw_script_html`
- `renderer_turns_frontmatter_into_metadata_block`
- `scanner_includes_mdx_files`

Keep these tests as the first renderer tests in `readmd`.

## Theme Settings

Theme should support these sections:

- Page:
  - background
  - padding
  - mobile padding

- Reader:
  - background
  - text color
  - muted text color
  - link color
  - width
  - border
  - shadow
  - radius

- Typography:
  - body font
  - heading font
  - code font
  - base font size
  - line height
  - paragraph gap

- Headings:
  - h1 size, weight, line height, margins
  - h2 size, weight, line height, margins
  - h3 size, weight, line height, margins
  - optional heading border

- Code:
  - inline code background
  - inline code text
  - code block background
  - code block text
  - code block border
  - code block radius
  - code block padding
  - syntax colors

- Tables:
  - border
  - header background
  - header text
  - row background
  - alternate row background
  - cell padding

- Blockquotes:
  - background
  - border color
  - text color
  - padding
  - style

- Lists:
  - marker color
  - indent
  - item gap

- Horizontal rule:
  - color
  - margin

## Decisions

- Name: `readmd`.
- Config format: TOML.
- Default output: standalone HTML file.
- `--tmp` creates a temporary standalone HTML file.
- `--open` opens the generated file after successful write.
- Default CSS mode: inline CSS.
- Main value: beautiful reading experience, not parser customization.
- First implementation should extract renderer logic from CalmPage `minimal-markdown-reader`.
- CLI should be separate from the Tauri app.

## Extraction Plan

1. Create Rust crate for `readmd`.
2. Copy CalmPage renderer functions into a library module, for example `src/renderer.rs`.
3. Remove Tauri-specific state, cache, vault scanning, watcher, and commands.
4. Keep renderer pure:
   - input: Markdown text and optional source path
   - output: title plus rendered HTML fragment
5. Add HTML document wrapping separately:
   - renderer creates article HTML
   - document builder adds `<!doctype html>`, `<head>`, CSS, and `<body>`
6. Add theme CSS generator separately:
   - theme config becomes CSS
   - generated CSS wraps the renderer output
7. Port existing CalmPage renderer tests.
8. Add CLI-specific tests for output path, config lookup, and theme printing.
9. Add CLI-specific tests for `--tmp`, `--open`, and invalid flag combinations.

## First Milestone

1. Create Rust CLI skeleton.
2. Add argument parser.
3. Add config loading.
4. Add built-in themes.
5. Extract Markdown renderer from CalmPage.
6. Generate standalone HTML.
7. Add `themes list`.
8. Add `themes print <name>`.
9. Add `config print-default`.
10. Add `--tmp` output.
11. Add `--open` after successful generation.
12. Add tests for output path, temp output, config merge, theme CSS, and invalid flags.

## Open Questions

- Should `readmd` allow raw HTML by default?
- Should code highlighting be built in from day one?
- Should image paths be copied, embedded, or left as relative links?
- Should custom CSS be allowed in config?
