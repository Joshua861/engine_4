#![allow(clippy::new_ret_no_self)]

use super::*;

mod active_fill;
pub use active_fill::*;

mod align;
pub use align::*;

mod aspect_ratio;
pub use aspect_ratio::*;

mod border;
pub use border::*;

mod box_fill;
pub use box_fill::*;

mod button;
pub use button::*;

mod center;
pub use center::*;

mod chart;
pub use chart::*;

mod circle;
pub use circle::*;

mod col;
pub use col::*;

mod drawer;
pub use drawer::*;

mod empty;
pub use empty::*;

mod fill;
pub use fill::*;

mod fit;
pub use fit::*;

mod flex;
pub use flex::*;

mod gradient_fill;
pub use gradient_fill::*;

mod grid;
pub use grid::*;

mod hoverable;
pub use hoverable::*;

mod image;
pub use image::*;

mod inactive_overlay;
pub use inactive_overlay::*;

mod input;
pub use input::*;

mod loading_bar;
pub use loading_bar::*;

mod padding;
pub use padding::*;

mod progress_bar;
pub use progress_bar::*;

mod rounded_fill;
pub use rounded_fill::*;

mod row;
pub use row::*;

mod scissor_box;
pub use scissor_box::*;

mod scroll;
pub use scroll::*;

mod selectbox;
pub use selectbox::*;

mod sized_box;
pub use sized_box::*;

mod slider;
pub use slider::*;

mod stack;
pub use stack::*;

mod text;
pub use text::*;

mod window;
pub use window::*;

mod data_input;
pub use data_input::*;

mod debug;
pub use debug::*;

mod hyperlink;
pub use hyperlink::*;

mod tooltip;
pub use tooltip::*;

mod modal;
pub use modal::*;

mod drop_file;
pub use drop_file::*;

mod search;
pub use search::*;

mod flow;
pub use flow::*;

mod loading_spinner;
pub use loading_spinner::*;

mod dock;
pub use dock::*;

use sge_window::window_width;

pub fn media_query<T>(small: T, medium: T, large: T) -> T {
    let width = window_width();

    if width < 600.0 {
        small
    } else if width < 2000.0 {
        medium
    } else {
        large
    }
}
