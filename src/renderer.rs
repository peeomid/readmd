use pulldown_cmark::{Options, Parser, html};
use serde_yaml::Value as YamlValue;
use std::{fs, path::Path};

use crate::{
    config::Config,
    document::build_document,
    error::{ReadmdError, Result},
};

pub fn is_markdown(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| matches!(ext.to_ascii_lowercase().as_str(), "md" | "markdown" | "mdx"))
        .unwrap_or(false)
}

pub fn title_from_path(path: &str) -> String {
    Path::new(path)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(|stem| stem.replace(['-', '_'], " "))
        .filter(|title| !title.trim().is_empty())
        .unwrap_or_else(|| "Untitled".to_string())
}

pub fn split_frontmatter(markdown: &str) -> (Option<String>, &str) {
    let markdown = markdown.strip_prefix('\u{feff}').unwrap_or(markdown);
    let Some(rest) = markdown
        .strip_prefix("---\n")
        .or_else(|| markdown.strip_prefix("---\r\n"))
    else {
        return (None, markdown);
    };

    if let Some(end) = rest.find("\n---\n") {
        let yaml = &rest[..end];
        let body = &rest[end + 5..];
        return (
            Some(yaml.to_string()),
            body.trim_start_matches(['\n', '\r']),
        );
    }

    if let Some(end) = rest.find("\r\n---\r\n") {
        let yaml = &rest[..end];
        let body = &rest[end + 7..];
        return (
            Some(yaml.to_string()),
            body.trim_start_matches(['\n', '\r']),
        );
    }

    (None, markdown)
}

pub fn render_frontmatter(frontmatter: &YamlValue) -> String {
    let Some(map) = frontmatter.as_mapping() else {
        return String::new();
    };
    if map.is_empty() {
        return String::new();
    }

    let mut html = String::from(
        "<section class=\"frontmatter\"><div class=\"frontmatter-title\">Document details</div><dl class=\"frontmatter-list\">",
    );
    for (key, value) in map {
        let key = key.as_str().unwrap_or("field");
        let value = match value {
            YamlValue::String(text) => text.clone(),
            YamlValue::Number(num) => num.to_string(),
            YamlValue::Bool(flag) => flag.to_string(),
            YamlValue::Sequence(items) => items
                .iter()
                .map(|item| match item {
                    YamlValue::String(text) => text.clone(),
                    YamlValue::Number(num) => num.to_string(),
                    YamlValue::Bool(flag) => flag.to_string(),
                    other => other.as_str().unwrap_or("").to_string(),
                })
                .collect::<Vec<_>>()
                .join(", "),
            other => other.as_str().unwrap_or("").to_string(),
        };
        if value.is_empty() {
            continue;
        }
        html.push_str("<div class=\"frontmatter-item\"><dt>");
        html.push_str(&html_escape::encode_text(key));
        html.push_str("</dt><dd>");
        html.push_str(&html_escape::encode_text(&value));
        html.push_str("</dd></div>");
    }
    html.push_str("</dl></section>");
    html
}

pub fn render_markdown(markdown: &str) -> String {
    let (frontmatter, body_markdown) = split_frontmatter(markdown);

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(body_markdown, options);
    let mut body = String::new();
    html::push_html(&mut body, parser);

    let frontmatter_html = frontmatter
        .and_then(|yaml| serde_yaml::from_str::<YamlValue>(&yaml).ok())
        .map(|value| render_frontmatter(&value))
        .unwrap_or_default();

    let combined = if frontmatter_html.is_empty() {
        body
    } else {
        format!("{frontmatter_html}{body}")
    };

    ammonia::Builder::default()
        .add_tags(["input"])
        .add_tags(["section", "dl", "dt", "dd", "div"])
        .add_generic_attributes(["class", "checked", "disabled", "id", "type"])
        .clean(&combined)
        .to_string()
}

pub fn note_title(markdown: &str, note_path: &str) -> String {
    let (frontmatter, body_markdown) = split_frontmatter(markdown);
    if let Some(yaml) = frontmatter
        .and_then(|yaml| serde_yaml::from_str::<YamlValue>(&yaml).ok())
        .and_then(|value| value.as_mapping().cloned())
    {
        if let Some(title) = yaml
            .get(&YamlValue::String("title".to_string()))
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|title| !title.is_empty())
        {
            return title.to_string();
        }
    }

    body_markdown
        .lines()
        .find_map(|line| line.trim().strip_prefix("# ").map(str::trim))
        .filter(|title| !title.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| title_from_path(note_path))
}

pub fn render(
    input: &Path,
    config: &Config,
    theme_name: Option<&str>,
    style_name: Option<&str>,
) -> Result<String> {
    if !is_markdown(input) {
        return Err(ReadmdError::Message(format!(
            "input is not a supported Markdown file: {}",
            input.display()
        )));
    }

    let markdown = fs::read_to_string(input).map_err(|source| ReadmdError::ReadFile {
        path: input.to_path_buf(),
        source,
    })?;
    let selected_theme = theme_name.unwrap_or(&config.default_theme);
    let selected_style = style_name.unwrap_or(&config.default_style);
    let theme = config
        .theme_with_style(selected_theme, selected_style)
        .ok_or_else(|| ReadmdError::UnknownTheme(selected_theme.to_string()))?;
    let note_path = input.to_string_lossy();
    let title = note_title(&markdown, &note_path);
    let article_html = render_markdown(&markdown);

    Ok(build_document(&title, &theme.css(), &article_html))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scanner_includes_mdx_files() {
        assert!(is_markdown(Path::new("note.mdx")));
        assert!(is_markdown(Path::new("NOTE.MDX")));
    }

    #[test]
    fn renderer_sanitizes_raw_script_html() {
        let html = render_markdown("# Safe\n\n<script>alert('x')</script>\n\n**bold**");

        assert!(html.contains("<h1>Safe</h1>"));
        assert!(html.contains("<strong>bold</strong>"));
        assert!(!html.contains("<script>"));
        assert!(!html.contains("alert"));
    }

    #[test]
    fn renderer_turns_frontmatter_into_metadata_block() {
        let markdown = "---\ntitle: CalmPage\nstatus: draft\ntags:\n  - markdown\n  - reader\n---\n# Ignore this heading\n\nBody text.";
        let html = render_markdown(markdown);

        assert!(html.contains("Document details"));
        assert!(html.contains("CalmPage"));
        assert!(html.contains("status"));
        assert!(html.contains("draft"));
        assert!(html.contains("markdown, reader"));
        assert!(html.contains("<h1>Ignore this heading</h1>"));
    }

    #[test]
    fn renderer_preserves_task_list_checkbox_type() {
        let html = render_markdown("- [ ] Research question\n- [x] Sources checked");

        assert!(html.contains("type=\"checkbox\""));
        assert!(html.contains("disabled"));
    }

    #[test]
    fn title_uses_frontmatter_title_first() {
        let markdown = "---\ntitle: From Frontmatter\n---\n# Body title";

        assert_eq!(note_title(markdown, "notes/example.md"), "From Frontmatter");
    }

    #[test]
    fn split_frontmatter_strips_bom_and_returns_body() {
        let (frontmatter, body) = split_frontmatter("\u{feff}---\ntitle: Hello\n---\n\nBody");

        assert_eq!(frontmatter.as_deref(), Some("title: Hello"));
        assert_eq!(body, "Body");
    }

    #[test]
    fn title_from_path_replaces_separators() {
        assert_eq!(title_from_path("folder/project_plan.md"), "project plan");
        assert_eq!(title_from_path("no-extension"), "no extension");
    }

    #[test]
    fn render_builds_full_document() {
        let dir = tempfile::tempdir().expect("temp dir");
        let input = dir.path().join("note.md");
        std::fs::write(&input, "# Hello").expect("write markdown");

        let html =
            render(&input, &crate::config::default_config(), None, None).expect("render html");

        assert!(html.starts_with("<!doctype html>"));
        assert!(html.contains("<title>Hello</title>"));
        assert!(html.contains("<article class=\"reader\"><h1>Hello</h1>"));
    }
}
