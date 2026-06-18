#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
INPUT="${1:-$ROOT/demos/source/deep-research-ai-techniques.md}"
OUT_DIR="$ROOT/demos/theme-style"
BIN="$ROOT/target/debug/readmd"

THEMES=(paper white graphite polar sepia midnight)
STYLES=(editorial notebook technical large)

if [[ ! -f "$INPUT" ]]; then
  echo "input markdown not found: $INPUT" >&2
  exit 1
fi

mkdir -p "$OUT_DIR"

cargo build --manifest-path "$ROOT/Cargo.toml"

for theme in "${THEMES[@]}"; do
  for style in "${STYLES[@]}"; do
    "$BIN" "$INPUT" \
      --theme "$theme" \
      --style "$style" \
      --output "$OUT_DIR/$theme-$style.html"
  done
done

cat > "$OUT_DIR/index.html" <<'HTML'
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>readmd theme and style demos</title>
    <style>
      :root {
        color-scheme: light;
        --bg: #f3eee4;
        --text: #25211b;
        --muted: #6f665b;
        --line: #d7cbbc;
        --panel: #fffaf1;
        --accent: #8b5e34;
      }

      * { box-sizing: border-box; }

      body {
        margin: 0;
        background: var(--bg);
        color: var(--text);
        font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
      }

      header {
        position: sticky;
        top: 0;
        z-index: 2;
        border-bottom: 1px solid var(--line);
        background: rgba(243, 238, 228, 0.94);
        backdrop-filter: blur(14px);
        padding: 22px 28px;
      }

      h1 {
        margin: 0 0 6px;
        font-size: 24px;
        line-height: 1.2;
      }

      p {
        margin: 0;
        color: var(--muted);
        line-height: 1.5;
      }

      main { padding: 24px 28px 42px; }

      .grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
        gap: 18px;
      }

      .card {
        overflow: hidden;
        border: 1px solid var(--line);
        border-radius: 8px;
        background: var(--panel);
      }

      .card-head {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 14px;
        padding: 12px 14px;
        border-bottom: 1px solid var(--line);
      }

      h2 {
        margin: 0;
        font-size: 15px;
        line-height: 1.3;
      }

      .tag {
        color: var(--muted);
        font-size: 12px;
        white-space: nowrap;
      }

      .preview {
        height: 250px;
        overflow: hidden;
        background: #e6ded2;
      }

      iframe {
        width: 1280px;
        height: 1000px;
        border: 0;
        transform: scale(0.26);
        transform-origin: 0 0;
        pointer-events: none;
      }

      .open-link {
        display: block;
        border-top: 1px solid var(--line);
        padding: 10px 14px;
        color: var(--accent);
        font-size: 13px;
        font-weight: 650;
        text-decoration: none;
      }

      .open-link:focus,
      .open-link:hover { background: #f6ead8; }
    </style>
  </head>
  <body>
    <header>
      <h1>readmd theme and style demos</h1>
      <p>Quick scan of every built-in color theme with every reading style.</p>
    </header>
    <main>
      <section class="grid" aria-label="Theme and style previews">
HTML

for theme in "${THEMES[@]}"; do
  title="$(tr '[:lower:]' '[:upper:]' <<< "${theme:0:1}")${theme:1}"
  for style in "${STYLES[@]}"; do
    file="$theme-$style.html"
    label="$(tr '[:lower:]' '[:upper:]' <<< "${style:0:1}")${style:1}"
    cat >> "$OUT_DIR/index.html" <<HTML
        <article class="card"><div class="card-head"><h2>$title</h2><span class="tag">$style</span></div><div class="preview"><iframe src="$file" title="$title $label preview"></iframe></div><a class="open-link" href="$file">Open full page</a></article>
HTML
  done
done

cat >> "$OUT_DIR/index.html" <<'HTML'
      </section>
    </main>
  </body>
</html>
HTML

echo "Generated demos in $OUT_DIR"
