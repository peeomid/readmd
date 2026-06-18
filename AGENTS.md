# AGENTS.md

## Summary

This repo is for `readmd`, a CLI that generates beautiful HTML reading pages from Markdown.

## Terms/context

- CLI: command line tool.
- Theme: named visual preset.
- Config: TOML file that defines visual settings.
- Agent-friendly help: help text that is explicit, stable, and easy for automation to parse.

## Overview

- Keep the product focused:
  - Markdown input.
  - HTML output.
  - Beautiful default reading experience.
  - Theme-based visual config.

- Do not expand into:
  - Blog engine.
  - Static site generator.
  - Markdown editor.
  - PDF generator.
  - Full document publishing system.

- Prefer Rust for the CLI.
- Prefer TOML for config.
- Keep commands simple and predictable.
- Make help text useful for agents:
  - Include purpose.
  - Include common examples.
  - Include config lookup order.
  - Include output behavior.
  - Include stable command names.

## Planned Commands

```bash
readmd <input.md>
readmd <input.md> --output <output.html>
readmd <input.md> --theme <name>
readmd <input.md> --config <readmd.toml>
readmd <input.md> --stdout
readmd themes list
readmd themes print <name>
readmd config print-default
```

## Coding Notes

- Renderer logic should live in a library module.
- CLI code should only handle args, config, file IO, and errors.
- Theme CSS generation should be testable without running the CLI.
- Keep default output portable: inline CSS unless user config says otherwise.

