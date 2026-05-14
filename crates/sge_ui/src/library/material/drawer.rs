use crate::{
    Child,
    base::{Padding, RoundedHoverFill, Row, drawer_state},
};

use super::*;

pub struct Drawer;

impl Drawer {
    pub fn new(id: usize, width: f32, title: impl ToString, contents: Child) -> UiRef {
        let scheme = scheme();
        let state = drawer_state(id);
        let icon = if state.open { "▼" } else { "▶" };

        crate::base::Drawer::new_full(
            RoundedHoverFill::new(
                scheme.surface_container,
                scheme.surface_container_high,
                CARD_RADIUS,
                Padding::all(
                    CARD_PADDING,
                    Row::with_gap(10.0, [Text::on_surface(icon), Text::on_surface(title)]),
                ),
            )
            .width(width)
            .fit_vertical(),
            Padding::all(CARD_PADDING, contents),
            false,
            id,
        )
        .rounded_fill(scheme.surface_container, CARD_RADIUS)
        .width(width)
        .fit_vertical()
    }
}
