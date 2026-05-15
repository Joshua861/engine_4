use sge_window::use_pointer_cursor_icon;

use super::*;

#[derive(Debug)]
pub struct Drawer {
    title: Child,
    contents: Child,
    default_open: bool,
    state: State<DrawerState>,
}

impl Drawer {
    pub fn new(title: Child, contents: Child, id: usize) -> UiRef {
        Self {
            title,
            contents,
            default_open: false,
            state: State::from_id(id),
        }
        .to_ref()
    }

    pub fn new_with_default_open(title: Child, contents: Child, id: usize) -> UiRef {
        Self {
            title,
            contents,
            default_open: true,
            state: State::from_id(id),
        }
        .to_ref()
    }

    pub fn new_full(title: Child, contents: Child, default_open: bool, id: usize) -> UiRef {
        Self {
            title,
            contents,
            default_open,
            state: State::from_id(id),
        }
        .to_ref()
    }

    fn state(&self) -> &'static mut DrawerState {
        self.state.get_or_create_mut(|| DrawerState {
            open: self.default_open,
        })
    }
}

#[derive(Debug, Default)]
pub struct DrawerState {
    pub open: bool,
}

impl UiNode for Drawer {
    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        let state = self.state();

        let title_size = self.title.draw(area, ui);
        let title_area = Area::new(area.top_left, title_size);

        if ui.is_clicked(title_area) {
            state.open = !state.open;
        }

        if ui.is_hovered(title_area) {
            use_pointer_cursor_icon();
        }

        if state.open {
            let contents_origin = area.top_left + Vec2::new(0.0, title_size.y);
            let remaining = area.size - Vec2::new(0.0, title_size.y);
            let remaining_area = Area::new(contents_origin, remaining);
            let contents_size_hint = self.contents.node.size(remaining_area);
            let contents_area = Area::new(
                contents_origin,
                Vec2::new(area.size.x, contents_size_hint.y),
            );
            let contents_size = self.contents.draw(contents_area, ui);
            title_size + Vec2::new(0.0, contents_size.y)
        } else {
            title_size
        }
    }

    fn preferred_dimensions(&self) -> Vec2 {
        let state = self.state();

        if state.open {
            Col::new([self.title, self.contents]).preferred_dimensions()
        } else {
            self.title.preferred_dimensions()
        }
    }

    fn size(&self, area: Area) -> Vec2 {
        let state = self.state();

        if state.open {
            Col::new([self.title, self.contents]).size(area)
        } else {
            self.title.size(area)
        }
    }
}

pub fn drawer_state(id: usize) -> &'static mut DrawerState {
    State::from_id(id).get_or_default()
}
