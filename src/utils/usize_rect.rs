use bevy_math::{IRect, Rect, USizeVec2};

/// A rectangle defined by two opposite corners.
///
/// The rectangle is axis aligned, and defined by its minimum and maximum coordinates,
/// stored in `USizeRect::min` and `USizeRect::max`, respectively. The minimum/maximum invariant
/// must be upheld by the user when directly assigning the fields, otherwise some methods
/// produce invalid results. It is generally recommended to use one of the constructor
/// methods instead, which will ensure this invariant is met, unless you already have
/// the minimum and maximum corners.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct USizeRect {
    /// The minimum corner point of the rect.
    pub min: USizeVec2,
    /// The maximum corner point of the rect.
    pub max: USizeVec2,
}

impl USizeRect {
    /// An empty `USizeRect`, represented by maximum and minimum corner points
    /// with `max == USizeVec2::MIN` and `min == USizeVec2::MAX`, so the
    /// rect has an extremely large negative size.
    /// This is useful, because when taking a union B of a non-empty `USizeRect` A and
    /// this empty `USizeRect`, B will simply equal A.
    pub const EMPTY: Self = Self {
        max: USizeVec2::MIN,
        min: USizeVec2::MAX,
    };
    /// Create a new rectangle from two corner points.
    ///
    /// The two points do not need to be the minimum and/or maximum corners.
    /// They only need to be two opposite corners.
    ///
    /// # Examples
    ///
    /// ```
    /// # use engine_4::USizeRect;
    /// let r = USizeRect::new(0, 4, 10, 6); // w=10 h=2
    /// let r = USizeRect::new(2, 4, 5, 0); // w=3 h=4
    /// ```
    #[inline]
    pub fn new(x0: usize, y0: usize, x1: usize, y1: usize) -> Self {
        Self::from_corners(USizeVec2::new(x0, y0), USizeVec2::new(x1, y1))
    }

    /// Create a new rectangle from two corner points.
    ///
    /// The two points do not need to be the minimum and/or maximum corners.
    /// They only need to be two opposite corners.
    ///
    /// # Examples
    ///
    /// ```
    /// # use engine_4::{USizeRect, USizeVec2};
    /// // Unit rect from [0,0] to [1,1]
    /// let r = USizeRect::from_corners(USizeVec2::ZERO, USizeVec2::ONE); // w=1 h=1
    /// // Same; the points do not need to be ordered
    /// let r = USizeRect::from_corners(USizeVec2::ONE, USizeVec2::ZERO); // w=1 h=1
    /// ```
    #[inline]
    pub fn from_corners(p0: USizeVec2, p1: USizeVec2) -> Self {
        Self {
            min: p0.min(p1),
            max: p0.max(p1),
        }
    }

    /// Create a new rectangle from its center and size.
    ///
    /// # Rounding Behavior
    ///
    /// If the size contains odd numbers they will be rounded down to the nearest whole number.
    ///
    /// # Panics
    ///
    /// This method panics if any of the components of the size is negative or if `origin - (size / 2)` results in any negatives.
    ///
    /// # Examples
    ///
    /// ```
    /// # use engine_4::{USizeRect, USizeVec2};
    /// let r = USizeRect::from_center_size(USizeVec2::ONE, USizeVec2::splat(2)); // w=2 h=2
    /// assert_eq!(r.min, USizeVec2::splat(0));
    /// assert_eq!(r.max, USizeVec2::splat(2));
    /// ```
    #[inline]
    pub fn from_center_size(origin: USizeVec2, size: USizeVec2) -> Self {
        assert!(
            origin.cmpge(size / 2).all(),
            "Origin must always be greater than or equal to (size / 2) otherwise the rectangle is undefined! Origin was {origin} and size was {size}"
        );
        let half_size = size / 2;
        Self::from_center_half_size(origin, half_size)
    }

    /// Create a new rectangle from its center and half-size.
    ///
    /// # Panics
    ///
    /// This method panics if any of the components of the half-size is negative or if `origin - half_size` results in any negatives.
    ///
    /// # Examples
    ///
    /// ```
    /// # use engine_4::{USizeRect, USizeVec2};
    /// let r = USizeRect::from_center_half_size(USizeVec2::ONE, USizeVec2::ONE); // w=2 h=2
    /// assert_eq!(r.min, USizeVec2::splat(0));
    /// assert_eq!(r.max, USizeVec2::splat(2));
    /// ```
    #[inline]
    pub fn from_center_half_size(origin: USizeVec2, half_size: USizeVec2) -> Self {
        assert!(
            origin.cmpge(half_size).all(),
            "Origin must always be greater than or equal to half_size otherwise the rectangle is undefined! Origin was {origin} and half_size was {half_size}"
        );
        Self {
            min: origin - half_size,
            max: origin + half_size,
        }
    }

    /// Check if the rectangle is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use engine_4::{USizeRect, USizeVec2};
    /// let r = USizeRect::from_corners(USizeVec2::ZERO, USizeVec2::new(0, 1)); // w=0 h=1
    /// assert!(r.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.min.cmpge(self.max).any()
    }

    /// Rectangle width (max.x - min.x).
    ///
    /// # Examples
    ///
    /// ```
    /// # use engine_4::USizeRect;
    /// let r = USizeRect::new(0, 0, 5, 1); // w=5 h=1
    /// assert_eq!(r.width(), 5);
    /// ```
    #[inline]
    pub const fn width(&self) -> usize {
        self.max.x - self.min.x
    }

    /// Rectangle height (max.y - min.y).
    ///
    /// # Examples
    ///
    /// ```
    /// # use engine_4::USizeRect;
    /// let r = USizeRect::new(0, 0, 5, 1); // w=5 h=1
    /// assert_eq!(r.height(), 1);
    /// ```
    #[inline]
    pub const fn height(&self) -> usize {
        self.max.y - self.min.y
    }

    /// Rectangle size.
    ///
    /// # Examples
    ///
    /// ```
    /// # use engine_4::{USizeRect, USizeVec2};
    /// let r = USizeRect::new(0, 0, 5, 1); // w=5 h=1
    /// assert_eq!(r.size(), USizeVec2::new(5, 1));
    /// ```
    #[inline]
    pub fn size(&self) -> USizeVec2 {
        self.max - self.min
    }

    /// Rectangle half-size.
    ///
    /// # Rounding Behavior
    ///
    /// If the full size contains odd numbers they will be rounded down to the nearest whole number when calculating the half size.
    ///
    /// # Examples
    ///
    /// ```
    /// # use engine_4::{USizeRect, USizeVec2};
    /// let r = USizeRect::new(0, 0, 4, 2); // w=4 h=2
    /// assert_eq!(r.half_size(), USizeVec2::new(2, 1));
    /// ```
    #[inline]
    pub fn half_size(&self) -> USizeVec2 {
        self.size() / 2
    }

    /// The center point of the rectangle.
    ///
    /// # Rounding Behavior
    ///
    /// If the (min + max) contains odd numbers they will be rounded down to the nearest whole number when calculating the center.
    ///
    /// # Examples
    ///
    /// ```
    /// # use engine_4::{USizeRect, USizeVec2};
    /// let r = USizeRect::new(0, 0, 4, 2); // w=4 h=2
    /// assert_eq!(r.center(), USizeVec2::new(2, 1));
    /// ```
    #[inline]
    pub fn center(&self) -> USizeVec2 {
        (self.min + self.max) / 2
    }

    /// Check if a point lies within this rectangle, inclusive of its edges.
    ///
    /// # Examples
    ///
    /// ```
    /// # use engine_4::USizeRect;
    /// let r = USizeRect::new(0, 0, 5, 1); // w=5 h=1
    /// assert!(r.contains(r.center()));
    /// assert!(r.contains(r.min));
    /// assert!(r.contains(r.max));
    /// ```
    #[inline]
    pub fn contains(&self, point: USizeVec2) -> bool {
        (point.cmpge(self.min) & point.cmple(self.max)).all()
    }

    /// Build a new rectangle formed of the union of this rectangle and another rectangle.
    ///
    /// The union is the smallest rectangle enclosing both rectangles.
    ///
    /// # Examples
    ///
    /// ```
    /// # use engine_4::{USizeRect, USizeVec2};
    /// let r1 = USizeRect::new(0, 0, 5, 1); // w=5 h=1
    /// let r2 = USizeRect::new(1, 0, 3, 8); // w=2 h=4
    /// let r = r1.union(r2);
    /// assert_eq!(r.min, USizeVec2::new(0, 0));
    /// assert_eq!(r.max, USizeVec2::new(5, 8));
    /// ```
    #[inline]
    pub fn union(&self, other: Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    /// Build a new rectangle formed of the union of this rectangle and a point.
    ///
    /// The union is the smallest rectangle enclosing both the rectangle and the point. If the
    /// point is already inside the rectangle, this method returns a copy of the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// # use engine_4::{USizeRect, USizeVec2};
    /// let r = USizeRect::new(0, 0, 5, 1); // w=5 h=1
    /// let u = r.union_point(USizeVec2::new(3, 6));
    /// assert_eq!(u.min, USizeVec2::ZERO);
    /// assert_eq!(u.max, USizeVec2::new(5, 6));
    /// ```
    #[inline]
    pub fn union_point(&self, other: USizeVec2) -> Self {
        Self {
            min: self.min.min(other),
            max: self.max.max(other),
        }
    }

    /// Build a new rectangle formed of the intersection of this rectangle and another rectangle.
    ///
    /// The intersection is the largest rectangle enclosed in both rectangles. If the intersection
    /// is empty, this method returns an empty rectangle ([`USizeRect::is_empty()`] returns `true`), but
    /// the actual values of [`USizeRect::min`] and [`USizeRect::max`] are implementation-dependent.
    ///
    /// # Examples
    ///
    /// ```
    /// # use engine_4::{USizeRect, USizeVec2};
    /// let r1 = USizeRect::new(0, 0, 2, 2); // w=2 h=2
    /// let r2 = USizeRect::new(1, 1, 3, 3); // w=2 h=2
    /// let r = r1.intersect(r2);
    /// assert_eq!(r.min, USizeVec2::new(1, 1));
    /// assert_eq!(r.max, USizeVec2::new(2, 2));
    /// ```
    #[inline]
    pub fn intersect(&self, other: Self) -> Self {
        let mut r = Self {
            min: self.min.max(other.min),
            max: self.max.min(other.max),
        };
        // Collapse min over max to enforce invariants and ensure e.g. width() or
        // height() never return a negative value.
        r.min = r.min.min(r.max);
        r
    }

    /// Create a new rectangle by expanding it evenly on all sides.
    ///
    /// A positive expansion value produces a larger rectangle,
    /// while a negative expansion value produces a smaller rectangle.
    /// If this would result in zero width or height, [`USizeRect::EMPTY`] is returned instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # use engine_4::{USizeRect, USizeVec2};
    /// let r = USizeRect::new(4, 4, 6, 6); // w=2 h=2
    /// let r2 = r.inflate(1); // w=4 h=4
    /// assert_eq!(r2.min, USizeVec2::splat(3));
    /// assert_eq!(r2.max, USizeVec2::splat(7));
    ///
    /// let r = USizeRect::new(4, 4, 8, 8); // w=4 h=4
    /// let r2 = r.inflate(-1); // w=2 h=2
    /// assert_eq!(r2.min, USizeVec2::splat(5));
    /// assert_eq!(r2.max, USizeVec2::splat(7));
    /// ```
    #[inline]
    pub fn inflate(&self, expansion: isize) -> Self {
        let mut r = Self {
            min: USizeVec2::new(
                self.min.x.saturating_add_signed(-expansion),
                self.min.y.saturating_add_signed(-expansion),
            ),
            max: USizeVec2::new(
                self.max.x.saturating_add_signed(expansion),
                self.max.y.saturating_add_signed(expansion),
            ),
        };
        // Collapse min over max to enforce invariants and ensure e.g. width() or
        // height() never return a negative value.
        r.min = r.min.min(r.max);
        r
    }

    /// Returns self as [`Rect`] (f32)
    #[inline]
    pub fn as_rect(&self) -> Rect {
        Rect::from_corners(self.min.as_vec2(), self.max.as_vec2())
    }

    /// Returns self as [`IRect`] (isize)
    #[inline]
    pub fn as_irect(&self) -> IRect {
        IRect::from_corners(self.min.as_ivec2(), self.max.as_ivec2())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn well_formed() {
        let r = USizeRect::from_center_size(USizeVec2::new(10, 16), USizeVec2::new(8, 12));

        assert_eq!(r.min, USizeVec2::new(6, 10));
        assert_eq!(r.max, USizeVec2::new(14, 22));

        assert_eq!(r.center(), USizeVec2::new(10, 16));

        assert_eq!(r.width(), 8);
        assert_eq!(r.height(), 12);
        assert_eq!(r.size(), USizeVec2::new(8, 12));
        assert_eq!(r.half_size(), USizeVec2::new(4, 6));

        assert!(r.contains(USizeVec2::new(7, 10)));
        assert!(r.contains(USizeVec2::new(14, 10)));
        assert!(r.contains(USizeVec2::new(10, 22)));
        assert!(r.contains(USizeVec2::new(6, 22)));
        assert!(r.contains(USizeVec2::new(14, 22)));
        assert!(!r.contains(USizeVec2::new(50, 5)));
    }

    #[test]
    fn rect_union() {
        let r = USizeRect::from_center_size(USizeVec2::splat(4), USizeVec2::splat(4)); // [2, 2] - [6, 6]

        // overlapping
        let r2 = USizeRect {
            min: USizeVec2::new(0, 0),
            max: USizeVec2::new(3, 3),
        };
        let u = r.union(r2);
        assert_eq!(u.min, USizeVec2::new(0, 0));
        assert_eq!(u.max, USizeVec2::new(6, 6));

        // disjoint
        let r2 = USizeRect {
            min: USizeVec2::new(4, 7),
            max: USizeVec2::new(8, 8),
        };
        let u = r.union(r2);
        assert_eq!(u.min, USizeVec2::new(2, 2));
        assert_eq!(u.max, USizeVec2::new(8, 8));

        // included
        let r2 = USizeRect::from_center_size(USizeVec2::splat(4), USizeVec2::splat(2));
        let u = r.union(r2);
        assert_eq!(u.min, r.min);
        assert_eq!(u.max, r.max);

        // including
        let r2 = USizeRect::from_center_size(USizeVec2::splat(4), USizeVec2::splat(6));
        let u = r.union(r2);
        assert_eq!(u.min, r2.min);
        assert_eq!(u.min, r2.min);
    }

    #[test]
    fn rect_union_pt() {
        let r = USizeRect::from_center_size(USizeVec2::splat(4), USizeVec2::splat(4)); // [2, 2] - [6, 6]

        // inside
        let v = USizeVec2::new(2, 5);
        let u = r.union_point(v);
        assert_eq!(u.min, r.min);
        assert_eq!(u.max, r.max);

        // outside
        let v = USizeVec2::new(10, 5);
        let u = r.union_point(v);
        assert_eq!(u.min, USizeVec2::new(2, 2));
        assert_eq!(u.max, USizeVec2::new(10, 6));
    }

    #[test]
    fn rect_intersect() {
        let r = USizeRect::from_center_size(USizeVec2::splat(6), USizeVec2::splat(8)); // [2, 2] - [10, 10]

        // overlapping
        let r2 = USizeRect {
            min: USizeVec2::new(8, 8),
            max: USizeVec2::new(12, 12),
        };
        let u = r.intersect(r2);
        assert_eq!(u.min, USizeVec2::new(8, 8));
        assert_eq!(u.max, USizeVec2::new(10, 10));

        // disjoint
        let r2 = USizeRect {
            min: USizeVec2::new(12, 12),
            max: USizeVec2::new(14, 18),
        };
        let u = r.intersect(r2);
        assert!(u.is_empty());
        assert_eq!(u.width(), 0);

        // included
        let r2 = USizeRect::from_center_size(USizeVec2::splat(6), USizeVec2::splat(2));
        let u = r.intersect(r2);
        assert_eq!(u.min, r2.min);
        assert_eq!(u.max, r2.max);

        // including
        let r2 = USizeRect::from_center_size(USizeVec2::splat(6), USizeVec2::splat(10));
        let u = r.intersect(r2);
        assert_eq!(u.min, r.min);
        assert_eq!(u.max, r.max);
    }

    #[test]
    fn rect_inflate() {
        let r = USizeRect::from_center_size(USizeVec2::splat(6), USizeVec2::splat(6)); // [3, 3] - [9, 9]

        let r2 = r.inflate(2);
        assert_eq!(r2.min, USizeVec2::new(1, 1));
        assert_eq!(r2.max, USizeVec2::new(11, 11));
    }
}
