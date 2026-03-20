//! Simple markdown-to-Span parser for rendering AI responses with formatting.

use iced::font::{Family, Style, Weight};
use iced::{Color, Font};

/// A styled text segment parsed from markdown.
pub struct MdSpan {
    pub text: String,
    pub bold: bool,
    pub italic: bool,
    pub code: bool,
    pub heading: bool,
}

/// Parse a markdown string into a list of styled spans.
/// Handles: **bold**, *italic*, `code`, headings (#), and list items (- ).
pub fn parse(input: &str) -> Vec<MdSpan> {
    let mut spans = Vec::new();

    for (i, line) in input.lines().enumerate() {
        if i > 0 {
            spans.push(MdSpan {
                text: "\n".to_string(),
                bold: false,
                italic: false,
                code: false,
                heading: false,
            });
        }

        let (line, is_heading) = if let Some(rest) = line.strip_prefix("### ") {
            (rest, true)
        } else if let Some(rest) = line.strip_prefix("## ") {
            (rest, true)
        } else if let Some(rest) = line.strip_prefix("# ") {
            (rest, true)
        } else {
            (line, false)
        };

        let line = if let Some(rest) = line.strip_prefix("- ") {
            spans.push(MdSpan {
                text: "  \u{2022} ".to_string(),
                bold: false,
                italic: false,
                code: false,
                heading: false,
            });
            rest
        } else {
            line
        };

        parse_inline(line, is_heading, &mut spans);
    }

    spans
}

fn parse_inline(text: &str, heading: bool, spans: &mut Vec<MdSpan>) {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut buf = String::new();
    let mut bold = false;
    let mut italic = false;
    let mut code = false;

    while i < len {
        // Inline code
        if chars[i] == '`' && !code {
            if !buf.is_empty() {
                spans.push(MdSpan { text: buf.clone(), bold, italic, code: false, heading });
                buf.clear();
            }
            code = true;
            i += 1;
            continue;
        }
        if chars[i] == '`' && code {
            spans.push(MdSpan { text: buf.clone(), bold: false, italic: false, code: true, heading: false });
            buf.clear();
            code = false;
            i += 1;
            continue;
        }
        if code {
            buf.push(chars[i]);
            i += 1;
            continue;
        }

        // Bold: **text**
        if i + 1 < len && chars[i] == '*' && chars[i + 1] == '*' {
            if !buf.is_empty() {
                spans.push(MdSpan { text: buf.clone(), bold, italic, code: false, heading });
                buf.clear();
            }
            bold = !bold;
            i += 2;
            continue;
        }

        // Italic: *text*
        if chars[i] == '*' {
            if !buf.is_empty() {
                spans.push(MdSpan { text: buf.clone(), bold, italic, code: false, heading });
                buf.clear();
            }
            italic = !italic;
            i += 1;
            continue;
        }

        buf.push(chars[i]);
        i += 1;
    }

    if !buf.is_empty() {
        spans.push(MdSpan { text: buf, bold, italic, code, heading });
    }
}

/// Convert a MdSpan to an iced text::Span with the given theme colors.
pub fn to_iced_span<'a>(
    span: &MdSpan,
    text_color: Color,
    base_font: Font,
) -> iced::widget::text::Span<'a, (), Font> {
    let font = if span.code {
        Font {
            family: Family::Monospace,
            ..base_font
        }
    } else {
        Font {
            weight: if span.bold || span.heading { Weight::Bold } else { base_font.weight },
            style: if span.italic { Style::Italic } else { base_font.style },
            ..base_font
        }
    };

    let color = if span.code {
        Color { r: 0.55, g: 0.83, b: 0.78, a: 1.0 }
    } else {
        text_color
    };

    let mut iced_span = iced::widget::text::Span::new(span.text.clone())
        .color(color)
        .font(font);

    if span.heading {
        iced_span = iced_span.size(18);
    }

    iced_span
}
