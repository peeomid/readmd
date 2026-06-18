use html_escape::encode_text;

pub fn build_document(title: &str, css: &str, article_html: &str) -> String {
    format!(
        concat!(
            "<!doctype html>\n",
            "<html lang=\"en\">\n",
            "<head>\n",
            "<meta charset=\"utf-8\">\n",
            "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n",
            "<title>{title}</title>\n",
            "<style>{css}</style>\n",
            "</head>\n",
            "<body>\n",
            "<main class=\"page\"><article class=\"reader\">{article}</article></main>\n",
            "</body>\n",
            "</html>\n"
        ),
        title = encode_text(title),
        css = css,
        article = article_html,
    )
}

pub fn print_document(title: &str, css: &str, article_html: &str) -> String {
    build_document(title, css, article_html)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_output_includes_doctype_title_style_and_article() {
        let html = build_document("A <Title>", "body{color:red;}", "<h1>Hello</h1>");
        assert!(html.starts_with("<!doctype html>"));
        assert!(html.contains("<title>A &lt;Title&gt;</title>"));
        assert!(html.contains("<style>body{color:red;}</style>"));
        assert!(html.contains("<article class=\"reader\">"));
        assert!(html.contains("<h1>Hello</h1>"));
    }
}
