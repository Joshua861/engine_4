use rand::{
    RngExt,
    distr::{
        Distribution, StandardUniform,
        uniform::{SampleRange, SampleUniform},
    },
    rng,
    rngs::ThreadRng,
};
use sge_color::Color;
use sge_global::global;
use sge_vectors::{Vec2, Vec3, Vec4};

pub use rand as rand_rs;

pub use const_random;

pub struct RandomState {
    rng: ThreadRng,
    counter: usize,
}

global!(RandomState, random);

pub fn init() {
    set_random(RandomState {
        rng: rng(),
        counter: 0,
    });
    log::info!("Initialized rng");
}

pub fn rand<T>() -> T
where
    StandardUniform: Distribution<T>,
{
    get_random().rng.random()
}

pub fn rand_usize() -> usize {
    get_random().rng.random::<u64>() as usize
}

/// generates f32 between -1 and 1
pub fn rand_f32() -> f32 {
    get_random().rng.random_range(-1.0..1.0)
}

/// Return a bool with a probability `p` of being true.
pub fn rand_bool(p: f64) -> bool {
    get_random().rng.random_bool(p)
}

pub fn rand_range<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    get_random().rng.random_range(range)
}

/// Return a bool with a probability of `numerator/denominator` of being
/// true.
pub fn rand_ratio(numerator: u32, denominator: u32) -> bool {
    get_random().rng.random_ratio(numerator, denominator)
}

/// returns the number one more than what it returned last time you called it
pub fn get_next_counter() -> usize {
    let state = get_random();
    let n = state.counter;
    state.counter += 1;
    n
}

pub fn rand_color() -> Color {
    Color::new(rand(), rand(), rand())
}

pub fn maybe_rand_choice<T>(choices: &[T]) -> Option<&T> {
    if choices.is_empty() {
        None
    } else {
        Some(&choices[rand_range(0..choices.len())])
    }
}

pub fn rand_choice<T>(choices: &[T]) -> &T {
    maybe_rand_choice(choices).expect("Cannot choose from an empty slice")
}

pub fn rand_vec2() -> Vec2 {
    Vec2::new(rand_range(-1.0..1.0), rand_range(-1.0..1.0))
}

pub fn rand_vec3() -> Vec3 {
    Vec3::new(
        rand_range(-1.0..1.0),
        rand_range(-1.0..1.0),
        rand_range(-1.0..1.0),
    )
}

pub fn rand_vec4() -> Vec4 {
    Vec4::new(
        rand_range(-1.0..1.0),
        rand_range(-1.0..1.0),
        rand_range(-1.0..1.0),
        rand_range(-1.0..1.0),
    )
}

#[macro_export]
macro_rules! id {
    () => {
        $crate::const_random::const_random!(usize)
    };
    ($s: expr) => {{
        use ::std::hash::{DefaultHasher, Hash, Hasher};

        let id = $s;
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        hasher.finish() as usize
    }};
    ($($v:expr),+) => {{
        let mut s: u128 = 0;
        $(s += $crate::id!($v) as u128;)*
        $crate::id!(s)
    }};
}
