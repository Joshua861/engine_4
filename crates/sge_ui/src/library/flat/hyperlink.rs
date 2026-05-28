use sge_color::Color;
use sge_text::rich_text::{RichText, RichTextBlock};

use crate::{
    UiRef,
    base::{self, RichTextNode},
};

pub struct Hyperlink;

impl Hyperlink {
    pub fn new(href: impl ToString) -> UiRef {
        let rich_text = RichTextNode::new(RichText::new(vec![RichTextBlock::underlined(
            href.to_string(),
            Color::SKY_500,
        )]));

        base::Hyperlink::new(href, rich_text)
    }

    pub fn with_title(href: impl ToString, title: impl ToString) -> UiRef {
        let rich_text = RichTextNode::new(RichText::new(vec![RichTextBlock::underlined(
            title,
            Color::SKY_500,
        )]));

        base::Hyperlink::new(href, rich_text)
    }

    pub fn dark(href: impl ToString) -> UiRef {
        let rich_text = RichTextNode::new(RichText::new(vec![RichTextBlock::underlined(
            href.to_string(),
            Color::BLUE_700,
        )]));

        base::Hyperlink::new(href, rich_text)
    }

    pub fn with_title_dark(href: impl ToString, title: impl ToString) -> UiRef {
        let rich_text = RichTextNode::new(RichText::new(vec![RichTextBlock::underlined(
            title,
            Color::BLUE_500,
        )]));

        base::Hyperlink::new(href, rich_text)
    }
}
