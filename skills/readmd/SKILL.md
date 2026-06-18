---
name: readmd
description: Use this skill when converting Markdown files to clean standalone HTML reading pages with the readmd CLI, choosing readmd themes/styles, exporting readmd config, regenerating readmd demo pages, or troubleshooting readmd output.
---

# readmd

Use `readmd` to turn one Markdown file into one portable HTML reading page.

## Commands

Render beside the input file:

```bash
readmd note.md
```

Choose output:

```bash
readmd note.md --output note.html
```

Render and open:

```bash
readmd note.md --tmp --open
```

Print HTML to stdout:

```bash
readmd note.md --stdout
```

## Theme And Style

- Theme controls colors.
- Style controls typography, spacing, and content width.

List options:

```bash
readmd themes list
readmd styles list
```

Use options:

```bash
readmd note.md --theme white --style notebook
```

Current built-ins:

- Themes: `paper`, `white`, `graphite`, `polar`, `sepia`, `midnight`
- Styles: `editorial`, `notebook`, `technical`, `large`

## Config

Create user config:

```bash
mkdir -p ~/.config/readmd
readmd config print-default > ~/.config/readmd/config.toml
```

Edit user config:

```bash
open ~/.config/readmd/config.toml
```

Lookup order:

1. `--config <path>`
2. `./readmd.toml`
3. `~/.config/readmd/config.toml`
4. built-in defaults

Useful config keys:

```toml
default_theme = "paper"
default_style = "editorial"
generated_by_readmd = true
```

Footer flags:

```bash
readmd note.md --no-generated-by-readmd
readmd note.md --generated-by-readmd
```

## Demos

In the readmd repo, regenerate demo pages:

```bash
scripts/regenerate-demos.sh
```

Use a custom source Markdown file:

```bash
scripts/regenerate-demos.sh path/to/article.md
```

Rendered public demos:

- https://peeomid.github.io/readmd/demos/theme-style/
- https://peeomid.github.io/readmd/demos/theme-style/paper-notebook.html
- https://peeomid.github.io/readmd/demos/theme-style/white-notebook.html

## Checks

After changing readmd code:

```bash
cargo fmt --all -- --check
cargo test
```
