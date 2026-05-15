use sge_rendering::{d2::Renderer2D, dq2d, wdq2d};
use sge_types::Metaballs;
use shapes_2d::Shape2DExt;

pub mod area;
pub mod shapes_2d;

pub trait Drawable {
    fn draw_to(&self, renderer: Renderer2D);
    fn draw(&self) {
        self.draw_to(dq2d())
    }
    fn draw_world(&self) {
        self.draw_to(wdq2d())
    }
}

impl<T: Shape2DExt> Drawable for T {
    fn draw_to(&self, renderer: Renderer2D) {
        <Self as Shape2DExt>::draw_to(&self, renderer);
    }

    fn draw(&self) {
        self.draw_to(dq2d())
    }

    fn draw_world(&self) {
        if self.is_visible_in_world() {
            self.draw_to(wdq2d())
        }
    }
}

pub fn draw<T: Drawable>(v: &T) {
    v.draw()
}

pub fn draw_world<T: Drawable>(v: &T) {
    v.draw_world()
}

pub fn draw_to<T: Drawable>(v: &T, renderer: Renderer2D) {
    v.draw_to(renderer)
}

// impl Drawable for Metaballs {
//     fn draw_to(&self, mut renderer: Renderer2D) {
//         renderer.add_metaball_batch(self);
//     }
// }
