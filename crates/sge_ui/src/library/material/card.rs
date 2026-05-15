use sge_vectors::vec2;

use super::*;
use crate::base::{Fill, Padding};

pub struct Card;

impl Card {
    fn color(color: Color, child: UiRef) -> UiRef {
        Fill::rounded(color, CARD_RADIUS, Padding::all(CARD_PADDING, child))
    }

    pub fn surface_container(child: UiRef) -> UiRef {
        let scheme = scheme();
        Self::color(scheme.surface_container, child)
    }

    pub fn interactable(child: UiRef) -> UiRef {
        let scheme = scheme();
        Fill::builder()
            .color(scheme.surface_container)
            .hover_color(scheme.surface_container_high)
            .active_color(scheme.surface_container_highest)
            .corner_radius(CARD_RADIUS)
            .child(Padding::all(CARD_PADDING, child))
            .build()
    }

    pub fn outlined(child: UiRef) -> UiRef {
        let scheme = scheme();
        Fill::builder()
            .color(scheme.surface_container)
            .stroke_width(1.0)
            .stroke_color(scheme.outline)
            .corner_radius(CARD_RADIUS)
            .child(Padding::all(CARD_PADDING, child))
            .build()
    }

    pub fn elevated(child: UiRef) -> UiRef {
        let scheme = scheme();
        Fill::builder()
            .color(scheme.surface_container)
            .shadow_offset(vec2(0.0, 2.0))
            .shadow_radius(2.0)
            .shadow_color(scheme.outline)
            .corner_radius(CARD_RADIUS)
            .child(Padding::all(CARD_PADDING, child))
            .build()
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
