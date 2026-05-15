use std::fmt::Display;

use crate::prelude::*;

pub struct Drawer;

impl Drawer {
    pub fn new(title: impl Display, bg: Color, id: usize, contents: Child) -> UiRef {
        Self::new_arrows(title, contents, bg, id, false)
    }

    pub fn new_alt(title: impl Display, bg: Color, id: usize, contents: Child) -> UiRef {
        Self::new_plus_minus(title, contents, bg, id, false)
    }

    pub fn new_with_default_open(
        title: impl Display,
        bg: Color,
        id: usize,
        contents: Child,
    ) -> UiRef {
        Self::new_arrows(title, contents, bg, id, true)
    }

    pub fn new_alt_with_default_open(
        title: impl Display,
        bg: Color,
        id: usize,
        contents: Child,
    ) -> UiRef {
        Self::new_plus_minus(title, contents, bg, id, true)
    }

    fn new_arrows(
        title: impl Display,
        contents: Child,
        bg: Color,
        id: usize,
        default_open: bool,
    ) -> UiRef {
        Self::new_internal(title, contents, bg, id, default_open, "▼", "▶")
    }

    fn new_plus_minus(
        title: impl Display,
        contents: Child,
        bg: Color,
        id: usize,
        default_open: bool,
    ) -> UiRef {
        Self::new_internal(title, contents, bg, id, default_open, "−", "+")
    }

    fn new_internal(
        title: impl Display,
        contents: Child,
        bg: Color,
        id: usize,
        default_open: bool,
        open_icon: &str,
        closed_icon: &str,
    ) -> UiRef {
        let state: &'static mut DrawerState = State::from_id(id).get_or_default();
        let icon = if state.open { open_icon } else { closed_icon };

        crate::base::Drawer::new_full(
            Fill::builder()
                .color(bg)
                .hover_color(bg.lighten(0.1))
                .child(Padding::tblr(
                    10.0,
                    15.0,
                    10.0,
                    10.0,
                    Text::nowrap(format!("{}  {}", icon, title)),
                ))
                .build()
                .fit_vertical(),
            BoxFill::new(bg, Padding::all(10.0, contents)),
            default_open,
            id,
        )
        .fit_vertical()
    }
}
