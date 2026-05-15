use crate::{
    NumberValue, UiRef,
    base::{BoxFill, CircleFill, EMPTY, Fill, Text},
};

use super::{BG1, BG3, BG4, FG3};

pub struct Slider;

impl Slider {
    pub fn new<T: NumberValue>(value: &mut T, min: T, max: T, id: usize) -> UiRef {
        let bar = Fill::new(BG1, EMPTY)
            .min_height(10.0)
            .padding_vertical(10.0);
        let handle = Fill::builder()
            .color(BG3)
            .hover_color(BG4)
            .child(EMPTY)
            .build()
            .sized_wh(20.0, 30.0);
        crate::base::Slider::new(value, min, max, handle, bar, id)
    }

    pub fn alternate<T: NumberValue + ToString>(value: &mut T, min: T, max: T, id: usize) -> UiRef {
        let string: String = value.to_string().chars().take(4).collect();
        let bar = BoxFill::new(BG1, EMPTY)
            .min_height(10.0)
            .padding_vertical(10.0);
        let handle = Fill::builder()
            .color(BG3)
            .hover_color(BG4)
            .child(
                Text::mono_colored(string, FG3)
                    .padding_top(4.0)
                    .padding_left(5.0),
            )
            .build()
            .sized_wh(50.0, 30.0);
        crate::base::Slider::new(value, min, max, handle, bar, id)
    }

    pub fn rounded<T: NumberValue>(value: &mut T, min: T, max: T, id: usize) -> UiRef {
        let handle = CircleFill::new(BG4).sized_wh(30.0, 30.0);
        let bar = Fill::rounded(BG1, 5.0, EMPTY)
            .min_height(10.0)
            .padding_vertical(10.0);
        crate::base::Slider::new(value, min, max, handle, bar, id)
    }
}
