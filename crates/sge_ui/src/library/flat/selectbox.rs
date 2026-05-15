use sge_color::Color;

use crate::{
    UiRef,
    base::{Fill, Padding, Text, select_box_value},
};

pub struct SelectBox;

impl SelectBox {
    pub fn new(
        base_color: Color,
        selected_color: Color,
        id: usize,
        choices: impl IntoIterator<Item = UiRef>,
    ) -> UiRef {
        let n = select_box_value(id).unwrap_or(0);
        crate::base::SelectBox::new(
            id,
            choices
                .into_iter()
                .enumerate()
                .map(|(i, node)| Fill::new(if i == n { selected_color } else { base_color }, node))
                .collect::<Vec<_>>(),
        )
    }

    pub fn new_text<T: ToString>(
        base_color: Color,
        selected_color: Color,
        id: usize,
        choices: impl IntoIterator<Item = T>,
    ) -> UiRef {
        Self::new(
            base_color,
            selected_color,
            id,
            choices.into_iter().map(|s| {
                Padding::tblr(
                    10.0,
                    15.0,
                    20.0,
                    20.0,
                    Text::new(s.to_string()).width(300.0),
                )
            }),
        )
    }

    pub fn mono_text<T: ToString>(
        base_color: Color,
        selected_color: Color,
        id: usize,
        choices: impl IntoIterator<Item = T>,
    ) -> UiRef {
        Self::new(
            base_color,
            selected_color,
            id,
            choices.into_iter().map(|s| {
                Padding::tblr(
                    10.0,
                    15.0,
                    20.0,
                    20.0,
                    Text::mono(s.to_string()).width(300.0),
                )
            }),
        )
    }
}
