use super::*;

pub struct LoadingSpinner;

impl LoadingSpinner {
    pub fn primary() -> UiRef {
        let scheme = scheme();
        let color = scheme.primary;
        crate::base::LoadingSpinner::new(color).square(50.0)
    }
}
