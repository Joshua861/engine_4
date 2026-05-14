use sge_text::{SANS, SANS_DISPLAY};

use crate::base;

use super::*;

pub struct Text;

impl Text {
    pub fn display_large(text: impl ToString) -> UiRef {
        base::Text::new_full(
            text,
            SANS_DISPLAY,
            rem(3.563),
            scheme().on_background,
            true,
            1.123,
            true,
        )
    }

    pub fn display_medium(text: impl ToString) -> UiRef {
        base::Text::new_full(
            text,
            SANS_DISPLAY,
            rem(2.813),
            scheme().on_background,
            true,
            1.156,
            true,
        )
    }

    pub fn display_small(text: impl ToString) -> UiRef {
        base::Text::new_full(
            text,
            SANS_DISPLAY,
            rem(2.25),
            scheme().on_background,
            true,
            1.222,
            true,
        )
    }

    pub fn headline_large(text: impl ToString) -> UiRef {
        base::Text::new_full(
            text,
            SANS_DISPLAY,
            rem(2.0),
            scheme().on_background,
            true,
            1.25,
            true,
        )
    }

    pub fn headline_medium(text: impl ToString) -> UiRef {
        base::Text::new_full(
            text,
            SANS_DISPLAY,
            rem(1.75),
            scheme().on_background,
            true,
            1.286,
            true,
        )
    }

    pub fn headline_small(text: impl ToString) -> UiRef {
        base::Text::new_full(
            text,
            SANS_DISPLAY,
            rem(1.5),
            scheme().on_background,
            true,
            1.333,
            true,
        )
    }

    pub fn title_large(text: impl ToString) -> UiRef {
        base::Text::new_full(
            text,
            SANS_DISPLAY,
            rem(1.375),
            scheme().on_background,
            true,
            1.273,
            true,
        )
    }

    pub fn title_medium(text: impl ToString) -> UiRef {
        base::Text::new_full(
            text,
            SANS_DISPLAY,
            rem(1.0),
            scheme().on_background,
            true,
            1.5,
            true,
        )
    }

    pub fn title_small(text: impl ToString) -> UiRef {
        base::Text::new_full(
            text,
            SANS_DISPLAY,
            rem(0.875),
            scheme().on_background,
            true,
            1.429,
            true,
        )
    }

    pub fn color(text: impl ToString, color: Color) -> UiRef {
        base::Text::new_full(text, SANS, rem(1.0), color, true, 1.5, true)
    }

    pub fn on_background(text: impl ToString) -> UiRef {
        Self::color(text, scheme().on_background)
    }

    pub fn on_primary(text: impl ToString) -> UiRef {
        Self::color(text, scheme().on_primary)
    }

    pub fn on_tertiary(text: impl ToString) -> UiRef {
        Self::color(text, scheme().on_tertiary)
    }

    pub fn on_primary_container(text: impl ToString) -> UiRef {
        Self::color(text, scheme().on_primary_container)
    }

    pub fn on_secondary(text: impl ToString) -> UiRef {
        Self::color(text, scheme().on_secondary)
    }

    pub fn on_secondary_container(text: impl ToString) -> UiRef {
        Self::color(text, scheme().on_secondary_container)
    }

    pub fn on_tertiary_container(text: impl ToString) -> UiRef {
        Self::color(text, scheme().on_tertiary_container)
    }

    pub fn on_surface(text: impl ToString) -> UiRef {
        Self::color(text, scheme().on_surface)
    }
}
