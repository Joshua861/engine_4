use crate::{
    UiRef,
    base::{
        self, Align, Border, BorderStyle, BoxFill, Button, Col, EMPTY, ImageNode, Padding, Stack,
        Text, button_clicked_last_frame,
    },
    get_ui_textures,
};

pub struct Modal;

impl Modal {
    pub fn new(title: impl ToString, open: &mut bool, id: usize, contents: UiRef) -> UiRef {
        let should_close = button_clicked_last_frame(id);
        if should_close {
            *open = false;
        }

        let contents = Col::new([
            Border::bottom(
                BorderStyle::custom(2.0, super::BG3, base::BorderType::Rectangle),
                Stack::new([
                    Padding::tblr(10.0, 15.0, 10.0, 10.0, Text::nowrap(title)),
                    Align::center_right(
                        Button::new(id, ImageNode::from_texture(get_ui_textures().close))
                            .width(30.0)
                            .fill(super::BG0),
                    ),
                ]),
            )
            .height(40.0),
            Padding::all(10.0, contents),
        ])
        .min_width(600.0)
        .fill(super::BG0)
        .border(super::BG3, 2.0);

        base::Modal::new(
            contents,
            BoxFill::new(super::BG0.with_alpha(0.5), EMPTY),
            open,
        )
    }
}
