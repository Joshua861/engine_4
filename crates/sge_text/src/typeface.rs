use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Typeface {
    pub base: FontRef,
    pub bold: Option<FontRef>,
    pub italic: Option<FontRef>,
    pub bold_italic: Option<FontRef>,
    pub display: Option<FontRef>,
}

#[cfg(feature = "extra_fonts")]
pub const SANS_TYPEFACE: Typeface = Typeface {
    base: SANS,
    bold: Some(SANS_BOLD),
    italic: Some(SANS_ITALIC),
    bold_italic: Some(SANS_BOLD_ITALIC),
    display: Some(SANS_DISPLAY),
};

pub const MONO_TYPEFACE: Typeface = Typeface {
    base: MONO,
    bold: None,
    italic: None,
    bold_italic: None,
    display: None,
};

impl Typeface {
    pub fn get_font(&self, font_type: FontType) -> FontRef {
        match font_type {
            FontType::Regular => self.base,
            FontType::Bold => self.bold.unwrap_or(self.base),
            FontType::Italic => self.italic.unwrap_or(self.base),
            FontType::BoldItalic => self.bold_italic.unwrap_or(self.bold.unwrap_or(self.base)),
            FontType::Display => self.display.unwrap_or(self.bold.unwrap_or(self.base)),
        }
    }
}

#[cfg(feature = "extra_fonts")]
impl Default for Typeface {
    fn default() -> Self {
        SANS_TYPEFACE
    }
}

#[cfg(not(feature = "extra_fonts"))]
impl Default for Typeface {
    fn default() -> Self {
        MONO_TYPEFACE
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontType {
    Regular,
    Bold,
    Italic,
    BoldItalic,
    Display,
}

impl FontType {
    pub fn toggle_bold(&mut self) {
        *self = match self {
            Self::Regular => Self::Bold,
            Self::Italic => Self::BoldItalic,
            Self::Bold => Self::Regular,
            Self::BoldItalic => Self::Italic,
            Self::Display => Self::Bold,
        };
    }

    pub fn toggle_italic(&mut self) {
        *self = match self {
            Self::Regular => Self::Italic,
            Self::Bold => Self::BoldItalic,
            Self::Italic => Self::Regular,
            Self::BoldItalic => Self::Bold,
            Self::Display => Self::Italic,
        };
    }

    pub fn set_bold(&mut self, bold: bool) {
        match (&self, bold) {
            (Self::Regular, true) => *self = Self::Bold,
            (Self::Italic, true) => *self = Self::BoldItalic,
            (Self::Bold, false) => *self = Self::Regular,
            (Self::BoldItalic, false) => *self = Self::Italic,
            _ => {}
        }
    }

    pub fn set_italic(&mut self, italic: bool) {
        match (&self, italic) {
            (Self::Regular, true) => *self = Self::Italic,
            (Self::Bold, true) => *self = Self::BoldItalic,
            (Self::Italic, false) => *self = Self::Regular,
            (Self::BoldItalic, false) => *self = Self::Bold,
            _ => {}
        }
    }

    pub fn is_italic(&self) -> bool {
        matches!(self, Self::Italic | Self::BoldItalic)
    }

    pub fn is_bold(&self) -> bool {
        matches!(self, Self::Bold | Self::BoldItalic)
    }
}
