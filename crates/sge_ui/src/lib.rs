#![allow(clippy::new_ret_no_self)]
#![feature(trait_alias)]
#![feature(unsafe_cell_access)]

use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Add, Deref, DerefMut, Div, Mul, Sub},
};

use base::{Empty, FloatingWindow, Modal, Tooltip};
use glium::winit::event::MouseButton;
use num_traits::Zero;
use sge_color::Color;
use sge_input::{Input, get_input};
use sge_macros::{gen_ref_type, include_texture};
use sge_textures::{ImageFormat, TextureRef, load_texture_from_bytes_sync};
use sge_time::{delta_time, frame_count, time};
use sge_types::Area;
use sge_utils::{FromF32, PartialClamp, ToF32};
use sge_vectors::Vec2;
use sge_window::window_size;

/// base building blocks
pub mod base;
/// more complex/pre-styled widgets made of components
pub mod library;
pub mod prelude;

sge_global::global!(Textures, ui_textures);

struct Textures {
    pub close: TextureRef,
    pub minimise: TextureRef,
}

pub use sge_rng::id;

pub struct UiStorage {
    states: HashMap<StateRef, SomeState>,
    elements_interacted: HashMap<usize, usize>,
    elements_interacted_this_frame: Vec<usize>,
    windows: Vec<*const FloatingWindow>,
    tooltips: Vec<*const Tooltip>,
    modals: Vec<*const Modal>,
}

sge_global::global!(UiStorage, ui_storage);

pub fn init_ui() {
    let close = include_texture!("../assets/textures/close.png");
    let minimise = include_texture!("../assets/textures/minimise.png");

    set_ui_textures(Textures { close, minimise });

    init_ui_nodes_storage();
    set_ui_storage(UiStorage {
        states: HashMap::new(),
        elements_interacted: HashMap::new(),
        elements_interacted_this_frame: Vec::new(),
        windows: Vec::new(),
        tooltips: Vec::new(),
        modals: Vec::new(),
    });
    log::info!("Initialized sge_ui");
    update();

    set_ui_state(UiState::new());

    #[cfg(feature = "material")]
    library::material::init();
}

sge_global::global!(UiState, ui_state);

pub struct UiState {
    frame: usize,
    delta_time: f32,
    time: f32,
    consume_input: bool,
    consumed_input_last_frame: bool,
}

impl UiState {
    fn new() -> Self {
        Self {
            frame: frame_count(),
            delta_time: delta_time(),
            time: time(),
            consume_input: false,
            consumed_input_last_frame: maybe_get_ui_state()
                .map(|s| s.consume_input)
                .unwrap_or(false),
        }
    }

    pub fn consume_input(&self) {
        get_ui_state().consume_input = true;
    }

    pub fn input(&self) -> &'static Input {
        get_input()
    }

    pub fn frame(&self) -> usize {
        self.frame
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn is_hovered(&self, area: Area) -> bool {
        self.input().is_cursor_within_area(area)
    }

    pub fn is_active(&self, area: Area) -> bool {
        self.input().is_cursor_within_area(area) && self.input().mouse_held(MouseButton::Left)
    }

    pub fn is_clicked(&self, area: Area) -> bool {
        self.input().is_cursor_within_area(area) && self.input().mouse_released(MouseButton::Left)
    }
}

pub struct SomeNode {
    node: Box<dyn UiNode>,
}

impl Deref for SomeNode {
    type Target = dyn UiNode;

    fn deref(&self) -> &Self::Target {
        &*self.node
    }
}

impl DerefMut for SomeNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.node
    }
}

gen_ref_type!(SomeNode, UiRef, ui_nodes);

pub trait UiNode: Debug {
    fn preferred_dimensions(&self) -> Vec2;
    fn draw(&self, area: Area, ui: &UiState) -> Vec2;
    #[allow(unused_variables)]
    fn size(&self, area: Area) -> Vec2 {
        self.preferred_dimensions()
    }
}

pub trait ToGenericUiNode: UiNode + Sized {
    fn to_generic(self) -> SomeNode;
    fn to_ref(self) -> UiRef {
        self.to_generic().create()
    }
}

impl<T: UiNode + 'static> ToGenericUiNode for T {
    fn to_generic(self) -> SomeNode {
        SomeNode {
            node: Box::new(self),
        }
    }
}

pub(crate) type Child = UiRef;

impl Default for UiRef {
    fn default() -> Self {
        base::EMPTY
    }
}

/// run at start of frame
pub fn update() {
    for tooltip in get_ui_storage().tooltips.drain(..) {
        let tooltip = unsafe { &*tooltip };
        tooltip.actually_draw();
    }

    for window in get_ui_storage().windows.drain(..) {
        let window = unsafe { &*window };
        window.actually_draw(&UiState::new());
    }

    for modal in get_ui_storage().modals.drain(..) {
        let modal = unsafe { &*modal };
        modal.actually_draw(&UiState::new());
    }

    get_ui_nodes_state().clear();
    get_ui_nodes_state().push(Empty.to_generic());
    get_ui_storage().elements_interacted_this_frame.clear();

    set_ui_state(UiState::new());
}

pub fn ui_consumed_input() -> bool {
    get_ui_state().consume_input || get_ui_state().consumed_input_last_frame
}

/// does not limit ui elements to the edge of the screen.
///
/// be very careful with this. make sure you only put sized nodes in here
pub fn draw_ui_unbounded(node: UiRef, position: Vec2) -> Vec2 {
    let area = Area {
        top_left: position,
        size: Vec2::INFINITY,
    };

    draw_ui_in_area(node, area)
}

pub fn draw_ui(node: UiRef, position: Vec2) -> Vec2 {
    let area = Area {
        top_left: position,
        size: window_size() - position,
    };

    draw_ui_in_area(node, area)
}

pub fn draw_ui_window(node: UiRef) -> Vec2 {
    let area = Area {
        top_left: Vec2::ZERO,
        size: window_size(),
    };

    draw_ui_in_area(node, area)
}

pub fn draw_ui_in_area(node: UiRef, area: Area) -> Vec2 {
    node.node.draw(area, &get_ui_state())
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct State<T: Debug> {
    _ref: StateRef,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Debug + 'static> Debug for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateRef(usize);

impl<T: Debug> State<T> {
    pub fn from_id(id: usize) -> Self {
        Self {
            _ref: StateRef(id),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn to_id(&self) -> usize {
        self._ref.0
    }

    pub fn get_or_create_mut<F>(&self, create: F) -> &'static mut T
    where
        F: FnOnce() -> T,
    {
        let storage = &mut get_ui_storage().states;
        let some_state = &mut storage.get_mut(&self._ref);

        if let Some(state) = some_state
            && state.state.downcast_ref::<T>().is_none()
        {
            let new_state = create();
            state.state = Box::new(new_state);
        } else if some_state.is_none() {
            storage.insert(
                self._ref,
                SomeState {
                    state: Box::new(create()),
                },
            );
        }

        storage
            .get_mut(&self._ref)
            .unwrap()
            .state
            .downcast_mut::<T>()
            .unwrap()
    }

    pub fn get(&self) -> Option<&'static T> {
        let storage = &get_ui_storage().states;
        let some_state = storage.get(&self._ref)?;

        some_state.state.downcast_ref::<T>()
    }

    pub fn get_mut(&self) -> Option<&'static mut T> {
        let storage = &mut get_ui_storage().states;
        let some_state = storage.get_mut(&self._ref)?;

        some_state.state.downcast_mut::<T>()
    }
}

impl<T: Default + Debug> State<T> {
    pub fn get_or_default(&self) -> &'static mut T {
        self.get_or_create_mut(T::default)
    }
}

pub struct SomeState {
    state: Box<dyn std::any::Any>,
}

pub fn all_elements_interacted_this_frame() -> &'static [usize] {
    &get_ui_storage().elements_interacted_this_frame
}

pub trait NumberValue = Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Div<Self, Output = Self>
    + Mul<Self, Output = Self>
    + 'static
    + Sized
    + Debug
    + PartialOrd
    + Copy
    + ToF32
    + FromF32
    + Zero
    + PartialClamp;
