use super::*;
use crate::base::{Padding, RoundedFill};

pub struct Card;

impl Card {
    fn color(color: Color, child: UiRef) -> UiRef {
        RoundedFill::new(color, CARD_RADIUS, Padding::all(CARD_PADDING, child))
    }

    pub fn surface_container(child: UiRef) -> UiRef {
        let scheme = scheme();
        Self::color(scheme.surface_container, child)
    }

    pub fn surface_container_low(child: UiRef) -> UiRef {
        let scheme = scheme();
        Self::color(scheme.surface_container_low, child)
    }

    pub fn surface_container_high(child: UiRef) -> UiRef {
        let scheme = scheme();
        Self::color(scheme.surface_container_high, child)
    }

    pub fn surface_container_highest(child: UiRef) -> UiRef {
        let scheme = scheme();
        Self::color(scheme.surface_container_highest, child)
    }

    pub fn surface(child: UiRef) -> UiRef {
        let scheme = scheme();
        Self::color(scheme.surface, child)
    }

    pub fn surface_container_lowest(child: UiRef) -> UiRef {
        let scheme = scheme();
        Self::color(scheme.surface_container_lowest, child)
    }

    pub fn error(child: UiRef) -> UiRef {
        let scheme = scheme();
        Self::color(scheme.error_container, child)
    }

    pub fn primary(child: UiRef) -> UiRef {
        let scheme = scheme();
        Self::color(scheme.primary_container, child)
    }

    pub fn secondary(child: UiRef) -> UiRef {
        let scheme = scheme();
        Self::color(scheme.secondary_container, child)
    }

    pub fn tertiary(child: UiRef) -> UiRef {
        let scheme = scheme();
        Self::color(scheme.tertiary_container, child)
    }
}
