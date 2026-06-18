use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Theme {
    pub name: String,
    #[serde(default)]
    pub page: PageTheme,
    #[serde(default)]
    pub reader: ReaderTheme,
    #[serde(default)]
    pub typography: TypographyTheme,
    #[serde(default)]
    pub headings: HeadingsTheme,
    #[serde(default)]
    pub code: CodeTheme,
    #[serde(default)]
    pub syntax: SyntaxTheme,
    #[serde(default)]
    pub table: TableTheme,
    #[serde(default)]
    pub blockquote: BlockquoteTheme,
    #[serde(default)]
    pub list: ListTheme,
    #[serde(default)]
    pub hr: HorizontalRuleTheme,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PageTheme {
    pub background: String,
    pub padding: String,
    pub mobile_padding: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ReaderTheme {
    pub background: String,
    pub text: String,
    pub muted: String,
    pub link: String,
    pub width: String,
    pub border: String,
    pub shadow: String,
    pub radius: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TypographyTheme {
    pub body_font: String,
    pub heading_font: String,
    pub code_font: String,
    pub font_size: String,
    pub line_height: String,
    #[serde(default)]
    pub weight: u16,
    pub paragraph_gap: String,
    #[serde(default)]
    pub code_scale: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct HeadingsTheme {
    #[serde(default)]
    pub h1: HeadingTheme,
    #[serde(default)]
    pub h2: HeadingTheme,
    #[serde(default)]
    pub h3: HeadingTheme,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct HeadingTheme {
    pub size: String,
    pub weight: u16,
    pub line_height: String,
    pub margin_top: String,
    pub margin_bottom: String,
    #[serde(default)]
    pub border_bottom: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CodeTheme {
    pub inline_background: String,
    pub inline_text: String,
    pub block_background: String,
    pub block_text: String,
    pub border: String,
    pub radius: String,
    pub padding: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SyntaxTheme {
    pub keyword: String,
    pub string: String,
    pub comment: String,
    pub function: String,
    pub number: String,
    pub operator: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TableTheme {
    pub border: String,
    pub header_background: String,
    pub header_text: String,
    pub row_background: String,
    pub row_alt_background: String,
    pub cell_padding: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BlockquoteTheme {
    pub background: String,
    pub border: String,
    pub text: String,
    pub padding: String,
    pub style: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ListTheme {
    pub marker: String,
    pub indent: String,
    pub item_gap: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct HorizontalRuleTheme {
    pub color: String,
    pub margin: String,
}

impl Theme {
    pub fn css(&self) -> String {
        let reader_width = if self.reader.width.is_empty() {
            "72ch"
        } else {
            self.reader.width.as_str()
        };
        let body_font = self.typography.body_font.as_str();
        let heading_font = self.typography.heading_font.as_str();
        let code_font = self.typography.code_font.as_str();
        let font_size = self.typography.font_size.as_str();
        let line_height = self.typography.line_height.as_str();
        let weight = if self.typography.weight == 0 {
            430
        } else {
            self.typography.weight
        };
        let paragraph_gap = self.typography.paragraph_gap.as_str();
        let code_scale = if self.typography.code_scale.is_empty() {
            "0.84em"
        } else {
            self.typography.code_scale.as_str()
        };
        let h1 = &self.headings.h1;
        let h2 = &self.headings.h2;
        let h3 = &self.headings.h3;

        format!(
            concat!(
                "html,body{{margin:0;padding:0;}}\n",
                "body{{--canvas:{page_bg};--panel:{panel_bg};--panel-strong:{panel_strong};--reader:{reader_bg};--text:{text};--muted:{muted};--line:{border};--accent:{marker};--accent-strong:{link};--accent-soft:{accent_soft};--highlight:{highlight};--code-bg:{code_bg};background:var(--canvas);color:var(--text);font-family:{body_font};font-size:{font_size};font-weight:{weight};line-height:{line_height};text-rendering:optimizeLegibility;-webkit-font-smoothing:antialiased;}}\n",
                ".page{{min-height:100vh;padding:{page_padding};box-sizing:border-box;}}\n",
                "@media (max-width: 720px){{.page{{padding:{mobile_padding};}}}}\n",
                ".reader{{color:{text};max-width:{reader_width};margin:0 auto;padding:clamp(18px,3vw,40px) 0;box-sizing:border-box;font-family:{body_font};font-size:{font_size};font-weight:{weight};line-height:{line_height};letter-spacing:0;text-wrap:pretty;}}\n",
                ".reader ::selection{{background:var(--highlight);}}\n",
                ".reader :target{{scroll-margin-top:32px;}}\n",
                ".reader a{{color:{link};text-decoration-thickness:0.08em;text-underline-offset:0.18em;}}\n",
                ".reader .muted{{color:{muted};}}\n",
                ".reader p,.reader ul,.reader ol,.reader blockquote{{margin:0 0 {paragraph_gap};}}\n",
                ".reader h1,.reader h2,.reader h3,.reader h4,.reader h5,.reader h6{{font-family:{heading_font};color:color-mix(in srgb,{text} 96%,{marker});line-height:1.13;margin:1.7em 0 0.48em;letter-spacing:0;text-wrap:balance;}}\n",
                ".reader h1{{font-size:{h1_size};font-weight:{h1_weight};line-height:1.03;margin-top:{h1_mt};margin-bottom:{h1_mb};}}\n",
                ".reader h2{{font-size:{h2_size};font-weight:{h2_weight};line-height:{h2_line};margin-top:{h2_mt};margin-bottom:{h2_mb};padding-bottom:0;border-bottom:0;}}\n",
                ".reader h3{{color:{marker};font-size:{h3_size};font-weight:{h3_weight};line-height:{h3_line};margin-top:{h3_mt};margin-bottom:{h3_mb};}}\n",
                ".reader ul,.reader ol{{padding-left:1.1em;list-style-position:outside;}}\n",
                ".reader li{{margin:0.12em 0;display:list-item;}}\n",
                ".reader li + li{{margin-top:{item_gap};}}\n",
                ".reader li::marker{{color:{marker};}}\n",
                ".reader li:has(> input[type=\"checkbox\"]){{list-style:none;margin-left:-1.1em;}}\n",
                ".reader li input[type=\"checkbox\"]{{width:0.82em;height:0.82em;margin:0 0.42em 0 0;vertical-align:-0.1em;appearance:none;-webkit-appearance:none;border:1.5px solid color-mix(in srgb,{marker} 62%,{border});border-radius:999px;background:transparent;display:inline-grid;place-content:center;}}\n",
                ".reader li input[type=\"checkbox\"]:checked{{border-color:{marker};background:{marker};}}\n",
                ".reader li input[type=\"checkbox\"]:checked::before{{content:\"\";width:0.28em;height:0.5em;border-right:2px solid white;border-bottom:2px solid white;transform:translateY(-0.04em) rotate(45deg);}}\n",
                ".reader code{{font-family:{code_font};background:{inline_bg};color:{inline_text};padding:0.12em 0.32em;border-radius:0.34em;font-size:{code_scale};letter-spacing:0;}}\n",
                ".reader pre{{background:{block_bg};color:{block_text};border:1px solid {code_border};border-radius:16px;padding:1.1em;overflow:auto;margin:1.2em 0;}}\n",
                ".reader pre code{{background:transparent;color:inherit;padding:0;border-radius:0;}}\n",
                ".reader table{{width:100%;border-collapse:collapse;margin:1.2em 0;font-size:0.92em;}}\n",
                ".reader th,.reader td{{padding:{cell_padding};border-bottom:1px solid {table_border};vertical-align:top;text-align:left;}}\n",
                ".reader th{{background:transparent;color:{header_text};font-weight:700;}}\n",
                ".reader tbody tr:nth-child(odd){{background:{row_bg};}}\n",
                ".reader tbody tr:nth-child(even){{background:{row_alt_bg};}}\n",
                ".reader blockquote{{background:{quote_bg};color:{quote_text};border-left:4px solid {quote_border};border-radius:0 18px 18px 0;padding:{quote_padding};margin:calc({paragraph_gap} * 1.25) 0;}}\n",
                ".reader .frontmatter{{margin:0 0 1.2em;padding:0.95em 1em;border:1px solid color-mix(in srgb,{border} 76%,transparent);border-radius:16px;background:color-mix(in srgb,{reader_bg} 72%,transparent);}}\n",
                ".reader .frontmatter-title{{margin:0 0 0.75em;color:{muted};font-size:0.78em;font-weight:700;text-transform:uppercase;letter-spacing:0.12em;}}\n",
                ".reader .frontmatter-list{{display:flex;flex-direction:column;gap:0.7em;margin:0;}}\n",
                ".reader .frontmatter-item{{display:flex;flex-direction:column;gap:0.18em;margin:0;}}\n",
                ".reader .frontmatter-item dt{{margin:0;color:{muted};font-size:0.78em;font-weight:650;letter-spacing:0.02em;}}\n",
                ".reader .frontmatter-item dd{{margin:0;color:{text};font-size:0.96em;line-height:1.45;word-break:break-word;}}\n",
                ".reader img{{max-width:100%;height:auto;border-radius:18px;box-shadow:0 14px 42px rgba(0,0,0,0.16);}}\n",
                ".reader hr{{width:34%;border:0;border-top:1px solid {hr_color};margin:{hr_margin};}}\n",
                ".readmd-footer{{max-width:{reader_width};margin:1.4em auto 0;padding:0 0 clamp(18px,3vw,40px);color:{muted};font-family:{body_font};font-size:0.78em;line-height:1.4;text-align:center;}}\n",
                ".readmd-footer a{{color:{link};text-decoration-thickness:0.08em;text-underline-offset:0.18em;}}\n",
                ".reader .syntax-keyword{{color:{keyword};}}\n",
                ".reader .syntax-string{{color:{string};}}\n",
                ".reader .syntax-comment{{color:{comment};}}\n",
                ".reader .syntax-function{{color:{function};}}\n",
                ".reader .syntax-number{{color:{number};}}\n",
                ".reader .syntax-operator{{color:{operator};}}\n"
            ),
            page_bg = self.page.background,
            panel_bg = self.table.header_background,
            panel_strong = self.reader.background,
            accent_soft = self.blockquote.background,
            code_bg = self.code.block_background,
            text = self.reader.text,
            body_font = body_font,
            font_size = font_size,
            line_height = line_height,
            page_padding = self.page.padding,
            mobile_padding = self.page.mobile_padding,
            reader_bg = self.reader.background,
            reader_width = reader_width,
            border = self.reader.border,
            link = self.reader.link,
            muted = self.reader.muted,
            highlight = self.reader.shadow,
            weight = weight,
            paragraph_gap = paragraph_gap,
            code_scale = code_scale,
            heading_font = heading_font,
            h1_size = h1.size,
            h1_weight = h1.weight,
            h1_mt = h1.margin_top,
            h1_mb = h1.margin_bottom,
            h2_size = h2.size,
            h2_weight = h2.weight,
            h2_line = h2.line_height,
            h2_mt = h2.margin_top,
            h2_mb = h2.margin_bottom,
            h3_size = h3.size,
            h3_weight = h3.weight,
            h3_line = h3.line_height,
            h3_mt = h3.margin_top,
            h3_mb = h3.margin_bottom,
            code_font = code_font,
            inline_bg = self.code.inline_background,
            inline_text = self.code.inline_text,
            block_bg = self.code.block_background,
            block_text = self.code.block_text,
            code_border = self.code.border,
            table_border = self.table.border,
            header_text = self.table.header_text,
            row_bg = self.table.row_background,
            row_alt_bg = self.table.row_alt_background,
            cell_padding = self.table.cell_padding,
            quote_bg = self.blockquote.background,
            quote_border = self.blockquote.border,
            quote_text = self.blockquote.text,
            quote_padding = self.blockquote.padding,
            item_gap = self.list.item_gap,
            marker = self.list.marker,
            hr_color = self.hr.color,
            hr_margin = self.hr.margin,
            keyword = self.syntax.keyword,
            string = self.syntax.string,
            comment = self.syntax.comment,
            function = self.syntax.function,
            number = self.syntax.number,
            operator = self.syntax.operator,
        )
    }

    pub fn to_toml(&self) -> String {
        toml::to_string_pretty(self).expect("theme should serialize to TOML")
    }

    pub fn paper() -> Self {
        theme_from_calmpage(color_preset("paper"), reader_preset("editorial"))
    }

    pub fn night() -> Self {
        theme_from_calmpage(color_preset("graphite"), reader_preset("editorial"))
    }

    pub fn clear() -> Self {
        theme_from_calmpage(color_preset("polar"), reader_preset("editorial"))
    }

    pub fn ink() -> Self {
        theme_from_calmpage(color_preset("sepia"), reader_preset("editorial"))
    }

    pub fn technical() -> Self {
        theme_from_calmpage(color_preset("paper"), reader_preset("technical"))
    }
}

pub fn builtin_themes() -> BTreeMap<String, Theme> {
    let mut themes = BTreeMap::new();
    for color in COLOR_PRESET_IDS {
        for reader in READER_PRESET_IDS {
            let theme = theme_from_calmpage(color_preset(color), reader_preset(reader));
            themes.insert(format!("{color}-{reader}"), theme);
        }
    }
    for (alias, color, reader) in [
        ("paper", "paper", "editorial"),
        ("white", "white", "editorial"),
        ("graphite", "graphite", "editorial"),
        ("polar", "polar", "editorial"),
        ("sepia", "sepia", "editorial"),
        ("midnight", "midnight", "editorial"),
        ("night", "graphite", "editorial"),
        ("clear", "polar", "editorial"),
        ("ink", "sepia", "editorial"),
        ("technical", "paper", "technical"),
    ] {
        let theme = theme_from_calmpage(color_preset(color), reader_preset(reader));
        themes.insert(alias.to_string(), theme);
    }
    themes
}

pub fn primary_theme_names() -> Vec<String> {
    COLOR_PRESET_IDS
        .iter()
        .map(|name| name.to_string())
        .collect()
}

pub fn builtin_theme_names() -> Vec<String> {
    primary_theme_names()
}

pub fn builtin_style_names() -> Vec<String> {
    READER_PRESET_IDS
        .iter()
        .map(|name| name.to_string())
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct StyleToml {
    pub name: String,
    pub typography: TypographyTheme,
    pub headings: HeadingsTheme,
    pub reader_width: String,
}

pub fn print_theme_toml(theme: &Theme) -> String {
    theme.to_toml()
}

pub fn builtin_theme_toml(name: &str) -> Option<String> {
    builtin_themes().get(&theme_key(name)).map(Theme::to_toml)
}

pub fn builtin_style_toml(name: &str) -> Option<String> {
    if !READER_PRESET_IDS.contains(&name) {
        return None;
    }
    let reader = reader_preset(name);
    let theme = theme_from_calmpage(color_preset("paper"), reader);
    let style = StyleToml {
        name: reader.name.to_string(),
        typography: theme.typography,
        headings: theme.headings,
        reader_width: theme.reader.width,
    };
    toml::to_string_pretty(&style).ok()
}

pub fn theme_key(name: &str) -> String {
    name.to_lowercase()
}

const COLOR_PRESET_IDS: &[&str] = &["paper", "white", "graphite", "polar", "sepia", "midnight"];
const READER_PRESET_IDS: &[&str] = &["editorial", "notebook", "technical", "large"];

#[derive(Clone, Copy)]
struct CalmColorPreset {
    name: &'static str,
    canvas: &'static str,
    panel: &'static str,
    reader: &'static str,
    text: &'static str,
    muted: &'static str,
    line: &'static str,
    accent: &'static str,
    accent_strong: &'static str,
    highlight: &'static str,
    link: &'static str,
    inline_code_text: &'static str,
    inline_code_bg: &'static str,
    code_block_bg: &'static str,
    code_border: &'static str,
    blockquote_bg: &'static str,
    blockquote_border: &'static str,
    table_border: &'static str,
}

#[derive(Clone, Copy)]
struct CalmReaderPreset {
    name: &'static str,
    body_font: &'static str,
    heading_font: &'static str,
    font_size: u16,
    line_height: f32,
    measure: u16,
    weight: u16,
    h1_scale: f32,
    h2_scale: f32,
    h3_scale: f32,
    paragraph_spacing: f32,
    code_scale: f32,
}

fn color_preset(id: &str) -> CalmColorPreset {
    match id {
        "white" => CalmColorPreset {
            name: "White",
            canvas: "#fffaf1",
            panel: "#f5edde",
            reader: "#fffaf1",
            text: "#1f1a14",
            muted: "#6f6557",
            line: "rgba(48, 38, 25, 0.15)",
            accent: "#8f431e",
            accent_strong: "#b94d19",
            highlight: "rgba(214, 134, 44, 0.24)",
            link: "#a04619",
            inline_code_text: "#7b3519",
            inline_code_bg: "#f0dfc5",
            code_block_bg: "#efe2cb",
            code_border: "rgba(126, 83, 33, 0.2)",
            blockquote_bg: "rgba(143, 67, 30, 0.1)",
            blockquote_border: "#a95a2a",
            table_border: "rgba(48, 38, 25, 0.16)",
        },
        "graphite" => CalmColorPreset {
            name: "Graphite",
            canvas: "#10100f",
            panel: "#181715",
            reader: "#171614",
            text: "#f0e5d2",
            muted: "#b5a890",
            line: "rgba(240, 229, 210, 0.13)",
            accent: "#e0a15f",
            accent_strong: "#f0b46e",
            highlight: "rgba(224, 161, 95, 0.22)",
            link: "#f0b46e",
            inline_code_text: "#f4c183",
            inline_code_bg: "#30291f",
            code_block_bg: "#242119",
            code_border: "rgba(224, 161, 95, 0.18)",
            blockquote_bg: "rgba(224, 161, 95, 0.1)",
            blockquote_border: "#d99552",
            table_border: "rgba(240, 229, 210, 0.14)",
        },
        "polar" => CalmColorPreset {
            name: "Polar",
            canvas: "#e8edf0",
            panel: "#f5f7f8",
            reader: "#fbfcfc",
            text: "#172027",
            muted: "#64717b",
            line: "rgba(23, 32, 39, 0.13)",
            accent: "#245f73",
            accent_strong: "#0d7890",
            highlight: "rgba(74, 144, 162, 0.22)",
            link: "#0d7890",
            inline_code_text: "#0f6072",
            inline_code_bg: "#ddebf0",
            code_block_bg: "#e4ecef",
            code_border: "rgba(36, 95, 115, 0.16)",
            blockquote_bg: "rgba(36, 95, 115, 0.09)",
            blockquote_border: "#2d7890",
            table_border: "rgba(23, 32, 39, 0.13)",
        },
        "sepia" => CalmColorPreset {
            name: "Sepia",
            canvas: "#d8c7a8",
            panel: "#ecdfc5",
            reader: "#f6ead2",
            text: "#2a2118",
            muted: "#78664e",
            line: "rgba(65, 45, 24, 0.16)",
            accent: "#795028",
            accent_strong: "#9d6129",
            highlight: "rgba(188, 126, 45, 0.25)",
            link: "#8b5620",
            inline_code_text: "#75451d",
            inline_code_bg: "#ead5ad",
            code_block_bg: "#e8d4ae",
            code_border: "rgba(121, 80, 40, 0.18)",
            blockquote_bg: "rgba(121, 80, 40, 0.11)",
            blockquote_border: "#8b5a28",
            table_border: "rgba(65, 45, 24, 0.16)",
        },
        "midnight" => CalmColorPreset {
            name: "Midnight",
            canvas: "#0c1117",
            panel: "#111923",
            reader: "#0f1720",
            text: "#e7edf3",
            muted: "#a6b3be",
            line: "rgba(231, 237, 243, 0.13)",
            accent: "#7fb7d7",
            accent_strong: "#9bd1ee",
            highlight: "rgba(127, 183, 215, 0.23)",
            link: "#9bd1ee",
            inline_code_text: "#a8d8f1",
            inline_code_bg: "#17283a",
            code_block_bg: "#141f2d",
            code_border: "rgba(127, 183, 215, 0.18)",
            blockquote_bg: "rgba(127, 183, 215, 0.1)",
            blockquote_border: "#7fb7d7",
            table_border: "rgba(231, 237, 243, 0.14)",
        },
        _ => CalmColorPreset {
            name: "Paper",
            canvas: "#e9e0d0",
            panel: "#f5edde",
            reader: "#fffaf1",
            text: "#1f1a14",
            muted: "#6f6557",
            line: "rgba(48, 38, 25, 0.15)",
            accent: "#8f431e",
            accent_strong: "#b94d19",
            highlight: "rgba(214, 134, 44, 0.24)",
            link: "#a04619",
            inline_code_text: "#7b3519",
            inline_code_bg: "#f0dfc5",
            code_block_bg: "#efe2cb",
            code_border: "rgba(126, 83, 33, 0.2)",
            blockquote_bg: "rgba(143, 67, 30, 0.1)",
            blockquote_border: "#a95a2a",
            table_border: "rgba(48, 38, 25, 0.16)",
        },
    }
}

fn reader_preset(id: &str) -> CalmReaderPreset {
    match id {
        "notebook" => CalmReaderPreset {
            name: "Notebook",
            body_font: "\"Source Serif 4\", \"Iowan Old Style\", Georgia, serif",
            heading_font: "\"Source Serif 4\", \"Iowan Old Style\", Georgia, serif",
            font_size: 17,
            line_height: 1.62,
            measure: 72,
            weight: 420,
            h1_scale: 2.05,
            h2_scale: 1.48,
            h3_scale: 1.22,
            paragraph_spacing: 0.98,
            code_scale: 0.84,
        },
        "technical" => CalmReaderPreset {
            name: "Technical",
            body_font: "Outfit, ui-sans-serif, system-ui, sans-serif",
            heading_font: "Outfit, ui-sans-serif, system-ui, sans-serif",
            font_size: 16,
            line_height: 1.58,
            measure: 78,
            weight: 450,
            h1_scale: 1.85,
            h2_scale: 1.38,
            h3_scale: 1.16,
            paragraph_spacing: 0.9,
            code_scale: 0.86,
        },
        "large" => CalmReaderPreset {
            name: "Large",
            body_font: "Newsreader, \"Iowan Old Style\", Georgia, serif",
            heading_font: "Newsreader, \"Iowan Old Style\", Georgia, serif",
            font_size: 20,
            line_height: 1.72,
            measure: 62,
            weight: 430,
            h1_scale: 2.0,
            h2_scale: 1.42,
            h3_scale: 1.18,
            paragraph_spacing: 1.15,
            code_scale: 0.82,
        },
        _ => CalmReaderPreset {
            name: "Editorial",
            body_font: "Newsreader, \"Iowan Old Style\", Georgia, serif",
            heading_font: "Newsreader, \"Iowan Old Style\", Georgia, serif",
            font_size: 18,
            line_height: 1.68,
            measure: 66,
            weight: 430,
            h1_scale: 2.35,
            h2_scale: 1.65,
            h3_scale: 1.28,
            paragraph_spacing: 1.08,
            code_scale: 0.84,
        },
    }
}

fn theme_from_calmpage(color: CalmColorPreset, reader: CalmReaderPreset) -> Theme {
    Theme {
        name: format!("{} {}", color.name, reader.name),
        page: PageTheme {
            background: color.canvas.to_string(),
            padding: "42px".to_string(),
            mobile_padding: "18px".to_string(),
        },
        reader: ReaderTheme {
            background: color.reader.to_string(),
            text: color.text.to_string(),
            muted: color.muted.to_string(),
            link: color.link.to_string(),
            width: format!("{}ch", reader.measure),
            border: color.line.to_string(),
            shadow: color.highlight.to_string(),
            radius: "0".to_string(),
        },
        typography: TypographyTheme {
            body_font: reader.body_font.to_string(),
            heading_font: reader.heading_font.to_string(),
            code_font: "\"SF Mono\", ui-monospace, monospace".to_string(),
            font_size: format!("{}px", reader.font_size),
            line_height: trim_float(reader.line_height),
            weight: reader.weight,
            paragraph_gap: format!("{}em", trim_float(reader.paragraph_spacing)),
            code_scale: format!("{}em", trim_float(reader.code_scale)),
        },
        headings: HeadingsTheme {
            h1: HeadingTheme {
                size: format!(
                    "calc({}px * {})",
                    reader.font_size,
                    trim_float(reader.h1_scale)
                ),
                weight: 700,
                line_height: "1.03".to_string(),
                margin_top: "0".to_string(),
                margin_bottom: "0.48em".to_string(),
                border_bottom: None,
            },
            h2: HeadingTheme {
                size: format!(
                    "calc({}px * {})",
                    reader.font_size,
                    trim_float(reader.h2_scale)
                ),
                weight: 700,
                line_height: "1.13".to_string(),
                margin_top: "1.7em".to_string(),
                margin_bottom: "0.48em".to_string(),
                border_bottom: None,
            },
            h3: HeadingTheme {
                size: format!(
                    "calc({}px * {})",
                    reader.font_size,
                    trim_float(reader.h3_scale)
                ),
                weight: 700,
                line_height: "1.13".to_string(),
                margin_top: "1.7em".to_string(),
                margin_bottom: "0.48em".to_string(),
                border_bottom: None,
            },
        },
        code: CodeTheme {
            inline_background: color.inline_code_bg.to_string(),
            inline_text: color.inline_code_text.to_string(),
            block_background: color.code_block_bg.to_string(),
            block_text: color.text.to_string(),
            border: color.code_border.to_string(),
            radius: "16px".to_string(),
            padding: "1.1em".to_string(),
        },
        syntax: SyntaxTheme {
            keyword: color.accent_strong.to_string(),
            string: color.accent.to_string(),
            comment: color.muted.to_string(),
            function: color.link.to_string(),
            number: color.accent_strong.to_string(),
            operator: color.text.to_string(),
        },
        table: TableTheme {
            border: color.table_border.to_string(),
            header_background: color.panel.to_string(),
            header_text: color.text.to_string(),
            row_background: "transparent".to_string(),
            row_alt_background: "transparent".to_string(),
            cell_padding: "0.55em 0.7em".to_string(),
        },
        blockquote: BlockquoteTheme {
            background: color.blockquote_bg.to_string(),
            border: color.blockquote_border.to_string(),
            text: color.text.to_string(),
            padding: "0.9em 1.1em".to_string(),
            style: "left-border".to_string(),
        },
        list: ListTheme {
            marker: color.accent.to_string(),
            indent: "1.1em".to_string(),
            item_gap: "0.12em".to_string(),
        },
        hr: HorizontalRuleTheme {
            color: color.line.to_string(),
            margin: "2.4em auto".to_string(),
        },
    }
}

fn trim_float(value: f32) -> String {
    let text = format!("{value:.2}");
    text.trim_end_matches('0').trim_end_matches('.').to_string()
}

#[allow(dead_code)]
fn old_theme_from_spec(
    name: &str,
    page_background: &str,
    reader_background: &str,
    text: &str,
    muted: &str,
    link: &str,
    code_inline_background: &str,
    code_block_background: &str,
    code_block_text: &str,
    code_inline_text: &str,
    code_border: &str,
    table_background: &str,
    quote_text: &str,
    page_shadow_hint: &str,
) -> Theme {
    let _ = page_shadow_hint;
    let _ = table_background;
    let _ = code_block_text;
    let mut theme = theme_from_calmpage(color_preset("paper"), reader_preset("editorial"));
    theme.name = name.to_string();
    theme.page.background = page_background.to_string();
    theme.reader.background = reader_background.to_string();
    theme.reader.text = text.to_string();
    theme.reader.muted = muted.to_string();
    theme.reader.link = link.to_string();
    theme.code.inline_background = code_inline_background.to_string();
    theme.code.block_background = code_block_background.to_string();
    theme.code.inline_text = code_inline_text.to_string();
    theme.code.border = code_border.to_string();
    theme.blockquote.text = quote_text.to_string();
    theme
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_css_contains_reader_code_and_table_rules() {
        let css = Theme::paper().css();
        assert!(css.contains(".reader"));
        assert!(css.contains("pre"));
        assert!(css.contains("code"));
        assert!(css.contains("table"));
    }

    #[test]
    fn built_in_themes_include_requested_names() {
        let themes = builtin_themes();
        assert!(themes.contains_key("paper"));
        assert!(themes.contains_key("white"));
        assert!(themes.contains_key("night"));
        assert!(themes.contains_key("clear"));
        assert!(themes.contains_key("ink"));
        assert!(themes.contains_key("technical"));
    }

    #[test]
    fn theme_prints_as_toml() {
        let toml = print_theme_toml(&Theme::paper());
        assert!(toml.contains("[page]"));
        assert!(toml.contains("name = \"Paper Editorial\""));
    }

    #[test]
    fn style_prints_as_toml() {
        let toml = builtin_style_toml("notebook").expect("style toml");
        assert!(toml.contains("name = \"Notebook\""));
        assert!(toml.contains("[typography]"));
        assert!(toml.contains("reader_width = \"72ch\""));
    }
}
