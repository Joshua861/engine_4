use glium::Rect;
use sge_types::Area;

use crate::get_render_state;

pub fn get_scissor_stack() -> &'static mut Vec<Rect> {
    &mut get_render_state().scissor_stack
}

pub fn push_scissor(rect: impl Into<Area>) {
    let rect = rect.into().to_glium_rect();
    let stack = get_scissor_stack();

    let final_rect = if let Some(current) = stack.last() {
        intersect_rects(*current, rect)
    } else {
        rect
    };

    stack.push(final_rect);
}

pub fn pop_scissor() {
    let stack = &mut get_scissor_stack();
    stack.pop();
}

pub fn current_scissor() -> Option<Rect> {
    let stack = &get_scissor_stack();
    stack.last().copied()
}

pub fn clear_scissor_stack() {
    let stack = &mut get_scissor_stack();
    stack.clear();
}

fn intersect_rects(a: Rect, b: Rect) -> Rect {
    let a_right = a.left + a.width;
    let a_top = a.bottom + a.height;
    let b_right = b.left + b.width;
    let b_top = b.bottom + b.height;

    let left = a.left.max(b.left);
    let bottom = a.bottom.max(b.bottom);
    let right = a_right.min(b_right);
    let top = a_top.min(b_top);

    Rect {
        left,
        bottom,
        width: right.saturating_sub(left),
        height: top.saturating_sub(bottom),
    }
}
