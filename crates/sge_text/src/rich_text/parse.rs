use sge_color::{Color, str_to_color};
use thiserror::Error;

use super::{RichText, RichTextBlock, RichTextStyle};

#[derive(Debug)]
pub struct RichTextParseError {
    pub pos: usize,
    pub kind: RichTextParseErrorKind,
}

#[derive(Error, Debug)]
pub enum RichTextParseErrorKind {
    #[error("Unclosed tag: {0}")]
    UnclosedTag(String),
    #[error("Unclosed quote.")]
    UnclosedQuote,
    #[error("Unclosed block: {0}")]
    UnclosedBlock(String),
    #[error("Expected non-empty tag.")]
    EmptyTag,
    #[error("Expected non-empty arguement name.")]
    EmptyArguement,
    #[error("Expected token: `{0}` at character: {1}.")]
    ExpectedToken(char, usize),
    #[error("Unexpected token: `{0}`, expected: `{1}` at character: {2}.")]
    UnexpectedToken(char, char, usize),
    #[error("Invalid arguement value \"{0}\", expected an integer (whole number).")]
    InvalidArguementInteger(String),
    #[error("Invalid arguement value \"{0}\", expected a boolean (true/false).")]
    InvalidArguementBoolean(String),
    #[error("Invalid arguement value \"{0}\", expected a color.")]
    InvalidArguementColor(String),
    #[error("Unknown tag: `{0}`")]
    UnknownTag(String),
    #[error("Unknown arguement: `{0}`")]
    UnknownArguement(String),
    #[error("This tag does not take any arguements.")]
    NoArguementsExpected,
    #[error("This tag requires these arguement(s): {0:?}")]
    MissingRequiredArguements(&'static [&'static str]),
    #[error("Unexpected closing brace `>`")]
    UnexpectedClosingBrace,
    #[error("Expected closing tag: `</{0}>`")]
    ExpectedClosingTag(String),
}

pub(crate) struct RichTextParser {
    chars: Vec<char>,
    i: usize,
    blocks: Vec<RichTextBlock>,
    errors: Vec<RichTextParseError>,
}

impl RichTextParser {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            i: 0,
            blocks: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn run(mut self) -> Result<RichText, Vec<RichTextParseError>> {
        let style = RichTextStyle::default();
        self.parse_content(style, None);

        if self.errors.is_empty() {
            Ok(RichText::new(self.blocks))
        } else {
            Err(self.errors)
        }
    }

    fn parse_tag(&mut self, tag: String, mut style: RichTextStyle) {
        let tag_name = tag.split_whitespace().next().unwrap_or("").to_string();

        if tag_name.is_empty() {
            self.error(RichTextParseErrorKind::EmptyTag);
            return;
        }

        let args = Self::parse_args(&tag);

        match tag_name.as_str() {
            "font" => {
                for (name, value) in args {
                    match name.as_str() {
                        "size" => match self.parse_usize_arg(&value) {
                            Some(v) => style.font_size = v,
                            None => return,
                        },
                        "bold" => style.font_type.set_bold(self.parse_bool_arg(&value)),
                        "italic" => style.font_type.set_italic(self.parse_bool_arg(&value)),
                        "color" => match self.parse_color_arg(&value) {
                            Some(c) => style.color = c,
                            None => return,
                        },
                        "strikethrough" | "st" => match self.parse_color_arg(&value) {
                            Some(c) => style.strikethrough = Some(c),
                            None => return,
                        },
                        "no-strikethrough" | "nostrikethough" | "no-st" | "nost" => {
                            style.strikethrough = None
                        }
                        "underline" | "ul" => match self.parse_color_arg(&value) {
                            Some(c) => style.underline = Some(c),
                            None => return,
                        },
                        "no-underline" | "nounderline" | "no-ul" | "noul" => style.underline = None,
                        "outline" | "ol" => match self.parse_color_arg(&value) {
                            Some(c) => style.outline = Some(c),
                            None => return,
                        },
                        "hl" | "highlight" | "bg" => match self.parse_color_arg(&value) {
                            Some(c) => style.highlight = Some(c),
                            None => return,
                        },
                        "no-highlight" | "nohl" | "no-hl" | "nohighlight" | "nobg" | "no-bg" => {
                            style.highlight = None
                        }
                        _ => self.error(RichTextParseErrorKind::UnknownArguement(name)),
                    }
                }
            }
            "underline" | "ul" => {
                match self.parse_optional_color_arg(args, style.color, "underline") {
                    Some(c) => style.underline = Some(c),
                    None => return,
                }
            }
            "strikethrough" | "st" => {
                match self.parse_optional_color_arg(args, style.color, "strikethrough") {
                    Some(c) => style.strikethrough = Some(c),
                    None => return,
                }
            }
            "no-underline" | "nounderline" | "no-ul" | "noul" => {
                if self.expect_no_args(args).is_none() {
                    return;
                }
                style.underline = None;
            }
            "no-strikethrough" | "nostrikethough" | "no-st" | "nost" => {
                if self.expect_no_args(args).is_none() {
                    return;
                }
                style.strikethrough = None;
            }
            "bold" | "b" | "strong" => {
                if self.expect_no_args(args).is_none() {
                    return;
                }
                style.font_type.set_bold(true);
            }
            "italic" | "i" | "em" => {
                if self.expect_no_args(args).is_none() {
                    return;
                }
                style.font_type.set_italic(true);
            }
            "a" => match args.first() {
                Some((name, value)) if name == "href" => style.href = value.clone(),
                _ => {
                    self.error(RichTextParseErrorKind::MissingRequiredArguements(&["href"]));
                    return;
                }
            },
            "ol" | "outline" => match self.parse_optional_color_arg(args, style.color, "outline") {
                Some(c) => style.outline = Some(c),
                None => return,
            },
            "bg" | "hl" | "highlight" => {
                match self.parse_optional_color_arg(args, style.color, "highlight") {
                    Some(c) => style.highlight = Some(c),
                    None => return,
                }
            }
            "no-highlight" | "nohl" | "no-hl" | "nohighlight" | "nobg" | "no-bg" => {
                if self.expect_no_args(args).is_none() {
                    return;
                }
                style.highlight = None;
            }
            _ => {
                self.error(RichTextParseErrorKind::UnknownTag(tag_name.clone()));
                return;
            }
        }

        self.parse_content(style, Some(&tag_name));
    }

    fn parse_content(&mut self, style: RichTextStyle, closing_tag: Option<&str>) {
        loop {
            if self.i >= self.chars.len() {
                if let Some(tag) = closing_tag {
                    self.error(RichTextParseErrorKind::UnclosedBlock(tag.to_string()));
                }
                return;
            }

            if self.chars[self.i] == '<' && self.chars.get(self.i + 1) == Some(&'/') {
                if let Some(expected) = closing_tag {
                    let close: Vec<char> = format!("</{expected}>").chars().collect();
                    if self.chars[self.i..].starts_with(&close) {
                        self.i += close.len();
                        return;
                    }
                }
                self.i += 1;
                self.consume_until('>');
                self.error(RichTextParseErrorKind::UnexpectedClosingBrace);
                return;
            }

            if self.chars[self.i] == '<' {
                self.i += 1;
                let tag = self.consume_until('>');
                self.parse_tag(tag, style.clone());
                continue;
            }

            let start = self.i;
            while self.i < self.chars.len() && self.chars[self.i] != '<' {
                self.i += 1;
            }
            let text: String = self.chars[start..self.i].iter().collect();
            if !text.is_empty() {
                self.blocks.push(RichTextBlock {
                    text,
                    style: style.clone(),
                });
            }
        }
    }

    fn parse_args(tag: &str) -> Vec<(String, Option<String>)> {
        let rest = tag.trim_start_matches(|c: char| !c.is_whitespace());
        let mut chars = rest.chars().peekable();
        let mut args = vec![];

        loop {
            while chars.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
                chars.next();
            }

            if chars.peek().is_none() {
                break;
            }

            let mut key = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_whitespace() || c == '=' {
                    break;
                }
                key.push(c);
                chars.next();
            }

            if key.is_empty() {
                break;
            }

            while chars.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
                chars.next();
            }

            if chars.peek() != Some(&'=') {
                args.push((key, None));
                continue;
            }
            chars.next();

            let value = if chars.peek() == Some(&'"') || chars.peek() == Some(&'\'') {
                let quote = chars.next().unwrap();
                let mut v = String::new();
                loop {
                    match chars.next() {
                        Some(c) if c == quote => break,
                        Some(c) => v.push(c),
                        None => break,
                    }
                }
                v
            } else {
                let mut v = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_whitespace() {
                        break;
                    }
                    v.push(c);
                    chars.next();
                }
                v
            };

            args.push((key, Some(value)));
        }

        args
    }

    fn parse_optional_color_arg(
        &mut self,
        args: Vec<(String, Option<String>)>,
        default: Color,
        tag: &str,
    ) -> Option<Color> {
        if args.is_empty() {
            Some(default)
        } else if args.len() == 1 && args[0].0 == "color" {
            self.parse_color_arg(&args[0].1)
        } else {
            self.error(RichTextParseErrorKind::UnknownArguement(format!(
                "{tag} tag only accepts a single optional `color` arguement, got: {:?}",
                args
            )));
            None
        }
    }

    fn expect_no_args(&mut self, args: Vec<(String, Option<String>)>) -> Option<()> {
        if args.is_empty() {
            Some(())
        } else {
            self.error(RichTextParseErrorKind::NoArguementsExpected);
            None
        }
    }

    fn parse_color_arg(&mut self, value: &Option<String>) -> Option<Color> {
        match value.as_ref().and_then(|v| str_to_color(v)) {
            Some(c) => Some(c),
            None => {
                self.error(RichTextParseErrorKind::InvalidArguementColor(
                    value.clone().unwrap_or_else(|| String::from("nothing")),
                ));
                None
            }
        }
    }

    fn parse_bool_arg(&mut self, value: &Option<String>) -> bool {
        match value.as_ref().and_then(|v| v.parse::<bool>().ok()) {
            Some(b) => b,
            None => true,
        }
    }

    fn parse_usize_arg(&mut self, value: &Option<String>) -> Option<usize> {
        match value.as_ref().and_then(|v| v.parse::<usize>().ok()) {
            Some(n) => Some(n),
            None => {
                self.error(RichTextParseErrorKind::InvalidArguementInteger(
                    value.clone().unwrap_or_else(|| String::from("nothing")),
                ));
                None
            }
        }
    }

    fn consume_until(&mut self, target: char) -> String {
        let start = self.i;

        while let Some(&c) = self.chars.get(self.i) {
            if c == target {
                break;
            }
            self.i += 1;
        }

        let out: String = self.chars[start..self.i].iter().collect();

        if self.chars.get(self.i) == Some(&target) {
            self.i += 1;
        }

        out
    }

    fn error(&mut self, kind: RichTextParseErrorKind) {
        self.errors.push(RichTextParseError { pos: self.i, kind });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn parse_blocks(input: &str) -> Vec<RichTextBlock> {
        RichTextParser::new(input)
            .run()
            .unwrap_or_else(|errs| panic!("expected Ok, got errors: {errs:#?}"))
            .blocks
    }

    fn parse_errors(input: &str) -> Vec<RichTextParseError> {
        RichTextParser::new(input)
            .run()
            .expect_err("expected parse errors, but got Ok")
    }

    #[test]
    fn test_empty_string() {
        assert!(parse_blocks("").is_empty());
    }

    #[test]
    fn test_plain_text() {
        let blocks = parse_blocks("hello world");
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].text, "hello world");
        assert_eq!(blocks[0].style, RichTextStyle::default());
    }

    #[test]
    fn test_plain_text_preserves_whitespace() {
        let blocks = parse_blocks("  spaces  and\ttabs  ");
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].text, "  spaces  and\ttabs  ");
    }

    #[test]
    fn test_bold_tag() {
        let blocks = parse_blocks("<b>hello</b>");
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].text, "hello");
        assert!(blocks[0].style.font_type.is_bold());
        assert!(!blocks[0].style.font_type.is_italic());
    }

    #[test]
    fn test_bold_aliases() {
        for tag in &["b", "bold", "strong"] {
            let blocks = parse_blocks(&format!("<{tag}>x</{tag}>"));
            assert!(
                blocks[0].style.font_type.is_bold(),
                "tag <{tag}> should set bold"
            );
        }
    }

    #[test]
    fn test_italic_tag() {
        let blocks = parse_blocks("<i>hello</i>");
        assert_eq!(blocks.len(), 1);
        assert!(blocks[0].style.font_type.is_italic());
        assert!(!blocks[0].style.font_type.is_bold());
    }

    #[test]
    fn test_italic_aliases() {
        for tag in &["i", "italic", "em"] {
            let blocks = parse_blocks(&format!("<{tag}>x</{tag}>"));
            assert!(
                blocks[0].style.font_type.is_italic(),
                "tag <{tag}> should set italic"
            );
        }
    }

    #[test]
    fn test_tag_does_not_bleed_outside() {
        let blocks = parse_blocks("before <b>bold</b> after");
        assert_eq!(blocks.len(), 3);
        assert_eq!(blocks[0].text, "before ");
        assert!(!blocks[0].style.font_type.is_bold());
        assert_eq!(blocks[1].text, "bold");
        assert!(blocks[1].style.font_type.is_bold());
        assert_eq!(blocks[2].text, " after");
        assert!(!blocks[2].style.font_type.is_bold());
    }

    #[test]
    fn test_multiple_text_siblings_inside_tag() {
        let blocks = parse_blocks("<b>one two three</b>");
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].text, "one two three");
        assert!(blocks[0].style.font_type.is_bold());
    }

    #[test]
    fn test_text_then_tag_then_text_inside_tag() {
        let blocks = parse_blocks("<b>hello <i>world</i> end</b>");
        assert_eq!(blocks.len(), 3);

        assert_eq!(blocks[0].text, "hello ");
        assert!(blocks[0].style.font_type.is_bold());
        assert!(!blocks[0].style.font_type.is_italic());

        assert_eq!(blocks[1].text, "world");
        assert!(blocks[1].style.font_type.is_bold());
        assert!(blocks[1].style.font_type.is_italic());

        assert_eq!(blocks[2].text, " end");
        assert!(blocks[2].style.font_type.is_bold());
        assert!(!blocks[2].style.font_type.is_italic());
    }

    #[test]
    fn test_many_siblings_inside_tag() {
        let blocks = parse_blocks("<b>a <i>b</i> c <i>d</i> e</b>");
        let texts: Vec<&str> = blocks.iter().map(|b| b.text.as_str()).collect();
        assert_eq!(texts, vec!["a ", "b", " c ", "d", " e"]);

        assert!(blocks[0].style.font_type.is_bold());
        assert!(blocks[1].style.font_type.is_bold() && blocks[1].style.font_type.is_italic());
        assert!(blocks[2].style.font_type.is_bold());
        assert!(blocks[3].style.font_type.is_bold() && blocks[3].style.font_type.is_italic());
        assert!(blocks[4].style.font_type.is_bold());
    }

    #[test]
    fn test_complex_nesting() {
        let input = r#"<ul color="red5">You <st>can</st> nest <font color=red5 bold>styles</font> <noul>inside</noul> of eachother</ul>"#;
        let blocks = parse_blocks(input);

        let texts: Vec<&str> = blocks.iter().map(|b| b.text.as_str()).collect();
        assert_eq!(
            texts,
            vec![
                "You ",
                "can",
                " nest ",
                "styles",
                " ",
                "inside",
                " of eachother"
            ]
        );

        assert!(blocks[0].style.underline.is_some());
        assert!(blocks[0].style.strikethrough.is_none());

        assert!(blocks[1].style.underline.is_some());
        assert!(blocks[1].style.strikethrough.is_some());

        assert!(blocks[2].style.underline.is_some());
        assert!(blocks[2].style.strikethrough.is_none());

        assert!(blocks[3].style.underline.is_some());
        assert!(blocks[3].style.font_type.is_bold());

        assert!(blocks[4].style.underline.is_some());

        assert!(blocks[5].style.underline.is_none());

        assert!(blocks[6].style.underline.is_some());
    }

    #[test]
    fn test_noul_resets_underline_for_children_only() {
        let blocks = parse_blocks("<ul>outer <noul>inner</noul> back</ul>");
        assert_eq!(blocks.len(), 3);
        assert!(
            blocks[0].style.underline.is_some(),
            "'outer' should have underline"
        );
        assert!(
            blocks[1].style.underline.is_none(),
            "'inner' should have no underline"
        );
        assert!(
            blocks[2].style.underline.is_some(),
            "'back' should have underline restored"
        );
    }

    #[test]
    fn test_nost_resets_strikethrough_for_children_only() {
        let blocks = parse_blocks("<st>outer <nost>inner</nost> back</st>");
        assert_eq!(blocks.len(), 3);
        assert!(blocks[0].style.strikethrough.is_some());
        assert!(blocks[1].style.strikethrough.is_none());
        assert!(blocks[2].style.strikethrough.is_some());
    }

    #[test]
    fn test_font_size() {
        let blocks = parse_blocks("<font size=32>big</font>");
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].style.font_size, 32);
    }

    #[test]
    fn test_font_bold_attribute() {
        let blocks = parse_blocks("<font bold>text</font>");
        assert!(blocks[0].style.font_type.is_bold());
    }

    #[test]
    fn test_font_multiple_attributes() {
        let blocks = parse_blocks("<font bold italic size=16>text</font>");
        assert!(blocks[0].style.font_type.is_bold());
        assert!(blocks[0].style.font_type.is_italic());
        assert_eq!(blocks[0].style.font_size, 16);
    }

    #[test]
    fn test_font_tag_with_children() {
        let blocks = parse_blocks("<font bold>a <i>b</i> c</font>");
        let texts: Vec<&str> = blocks.iter().map(|b| b.text.as_str()).collect();
        assert_eq!(texts, vec!["a ", "b", " c"]);
        assert!(blocks[0].style.font_type.is_bold());
        assert!(blocks[1].style.font_type.is_bold() && blocks[1].style.font_type.is_italic());
        assert!(blocks[2].style.font_type.is_bold());
    }

    #[test]
    fn test_deep_nesting() {
        let blocks = parse_blocks("<b><i><ul>deep</ul></i></b>");
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].text, "deep");
        assert!(blocks[0].style.font_type.is_bold());
        assert!(blocks[0].style.font_type.is_italic());
        assert!(blocks[0].style.underline.is_some());
    }

    #[test]
    fn test_sequential_top_level_tags() {
        let blocks = parse_blocks("<b>bold</b> <i>italic</i> plain");
        let texts: Vec<&str> = blocks.iter().map(|b| b.text.as_str()).collect();
        assert_eq!(texts, vec!["bold", " ", "italic", " plain"]);
        assert!(blocks[0].style.font_type.is_bold());
        assert!(!blocks[1].style.font_type.is_bold());
        assert!(blocks[2].style.font_type.is_italic());
        assert!(!blocks[3].style.font_type.is_italic());
    }

    #[test]
    fn test_a_tag() {
        let blocks = parse_blocks(r#"<a href="https://example.com">click</a>"#);
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].text, "click");
        assert_eq!(
            blocks[0].style.href,
            Some("https://example.com".to_string())
        );
    }

    #[test]
    fn test_unknown_tag_produces_error() {
        let errors = parse_errors("<blorp>text</blorp>");
        assert!(!errors.is_empty());
        assert!(matches!(&errors[0].kind, RichTextParseErrorKind::UnknownTag(t) if t == "blorp"));
    }

    #[test]
    fn test_unclosed_tag_produces_error() {
        let errors = parse_errors("<b>no closing tag");
        assert!(!errors.is_empty());
        assert!(matches!(&errors[0].kind, RichTextParseErrorKind::UnclosedBlock(t) if t == "b"));
    }

    #[test]
    fn test_empty_tag_produces_error() {
        let errors = parse_errors("<>text</>");
        assert!(!errors.is_empty());
        assert!(matches!(&errors[0].kind, RichTextParseErrorKind::EmptyTag));
    }

    #[test]
    fn test_a_tag_without_href_produces_error() {
        let errors = parse_errors("<a>click</a>");
        assert!(!errors.is_empty());
        assert!(matches!(
            &errors[0].kind,
            RichTextParseErrorKind::MissingRequiredArguements(_)
        ));
    }

    #[test]
    fn test_bold_with_args_produces_error() {
        let errors = parse_errors("<b foo=bar>text</b>");
        assert!(!errors.is_empty());
        assert!(matches!(
            &errors[0].kind,
            RichTextParseErrorKind::NoArguementsExpected
        ));
    }

    #[test]
    fn test_font_invalid_size_produces_error() {
        let errors = parse_errors("<font size=abc>text</font>");
        assert!(!errors.is_empty());
        assert!(matches!(
            &errors[0].kind,
            RichTextParseErrorKind::InvalidArguementInteger(_)
        ));
    }

    #[test]
    fn test_unexpected_closing_brace_produces_error() {
        let errors = parse_errors("text </b> more");
        assert!(!errors.is_empty());
        assert!(matches!(
            &errors[0].kind,
            RichTextParseErrorKind::UnexpectedClosingBrace
        ));
    }
}
