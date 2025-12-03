pub mod usize_rect;

pub trait EngineCreate<R> {
    fn create(self) -> R;
}
