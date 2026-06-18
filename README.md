# readmd

`readmd` turns Markdown files into beautiful, readable HTML pages.

The goal is simple: write in Markdown, run one command, get a polished HTML page that feels good to read.

![readmd paper notebook demo](assets/demo-paper-notebook.png)

## Live Demos

- Overview: <https://peeomid.github.io/readmd/demos/theme-style/>
- Paper notebook: <https://peeomid.github.io/readmd/demos/theme-style/paper-notebook.html>
- White notebook: <https://peeomid.github.io/readmd/demos/theme-style/white-notebook.html>

## Focus

- Generate HTML from Markdown.
- Make the default output beautiful without setup.
- Focus on reading experience: typography, spacing, color, code, tables, quotes, and layout.
- Keep the CLI clear for both humans and agents.
- Use themes for colors and styles for typography/width.

## Example

```bash
readmd note.md
```

This writes:

```text
note.html
```

Choose output:

```bash
readmd note.md --output public/note.html
```

Use a theme:

```bash
readmd note.md --theme paper
```

Use a theme and style:

```bash
readmd note.md --theme paper --style notebook
```

Use a config file:

```bash
readmd note.md --config ./readmd.toml
```

Print HTML to terminal:

```bash
readmd note.md --stdout
```

Hide the generated footer:

```bash
readmd note.md --no-generated-by-readmd
```

## Theme Config

`readmd` ships with CalmPage-style color themes and reader styles.

- Theme means color preset.
- Style means typography, spacing, and content width.
- Built-in default: `paper` theme with `editorial` style.

Config lookup order:

1. `--config <file>`
2. `./readmd.toml`
3. `~/.config/readmd/config.toml`
4. Built-in defaults

Print the default config:

```bash
readmd config print-default
```

Write the default config to your user config file:

```bash
mkdir -p ~/.config/readmd
readmd config print-default > ~/.config/readmd/config.toml
```

Open it for manual edits:

```bash
open ~/.config/readmd/config.toml
```

Example default values:

```toml
default_theme = "paper"
default_style = "editorial"
generated_by_readmd = true
```

After that, normal `readmd note.md` runs will use the edited config unless `--config`, `--theme`, `--style`, `--generated-by-readmd`, or `--no-generated-by-readmd` overrides it.

## Theme Commands

Theme commands:

```bash
readmd themes list
readmd themes print paper
```

Style commands:

```bash
readmd styles list
readmd styles print notebook
```

These commands help humans and agents inspect the exact built-in settings.

## Default Themes And Styles

- Themes: `paper`, `white`, `graphite`, `polar`, `sepia`, `midnight`.
- Styles: `editorial`, `notebook`, `technical`, `large`.

## Demo Pages

The repo includes generated demo pages for every built-in theme/style pair.

Open the overview page:

```bash
open demos/theme-style/index.html
```

Open one full page:

```bash
open demos/theme-style/paper-notebook.html
```

Demo naming format:

```text
demos/theme-style/<theme>-<style>.html
```

Regenerate all demo pages and the overview index:

```bash
scripts/regenerate-demos.sh
```

Use a custom demo Markdown file:

```bash
scripts/regenerate-demos.sh path/to/article.md
```

## Output

Default output is a full HTML document:

- `<!doctype html>`
- `<head>` with title and CSS
- `<body>` with one readable article
- inline CSS by default, so the file is portable

## Design Rule

The CLI should not feel like a generic converter.

It should feel like:

```text
Markdown in -> beautiful reading page out
```
