pub use crate::{init, init_custom, next_frame, run_async};
pub use sge_api::{Drawable, draw, draw_world};
pub use sge_include_assets::sge_include_assets;
pub use sge_macros::main;

pub use anyhow;
pub use anyhow::Result as AResult;

pub mod persistence {
    pub use sge_persistence::{
        Error as PersistenceError, PartialLerp, Persistent, persistent, rkyv,
    };
}

pub mod post_processing {
    pub use sge_rendering::post_processing::{
        PostProcessingEffect, PostProcessingError, RenderFullscreenQuadError,
        add_post_processing_effect, bloom_screen, blur_screen, brighten_screen,
        chromatic_abberation_screen, contrast_screen, film_grain_screen, greyscale_screen,
        hue_rotate_screen, invert_screen, pixelate_screen, saturate_screen, sharpen_screen,
        vignette_screen,
    };
}

pub mod storage {
    pub use crate::api::{
        storage_exists, storage_get_state, storage_get_state_mut, storage_init_state,
        storage_store_state, storage_try_get_state, storage_try_get_state_mut,
    };
}

pub mod image {
    pub use ::image as image_rs;
    pub use image_rs::ImageFormat;
    pub use sge_image::{Image, ImageRef, LoadImageError, load_image_sync};
}

pub mod physics {
    pub use sge_physics::{
        Bounds, ColliderConfig, CollisionPoints, ObjectRef, PhysicsWorld, WorldRef,
        player::PlayerBindBuilder, player::PlayerController,
    };
}

pub mod particles {
    pub use sge_particles::*;
}

pub mod config {
    pub use sge_config::{
        EngineConfig, EngineCreationOptions, Opts, OptsBuilder, dont_wait_for_events, get_config,
        get_polygon_mode, get_wait_for_events, get_wait_for_events_mut, set_magnify_filter,
        set_minify_filter, set_polygon_mode, set_wait_for_events, toggle_wait_for_events,
        toggle_wireframe, use_default_filtering, use_linear_filtering, use_mipmaps,
        use_nearest_filtering, use_positive_y_down, use_positive_y_up, wait_for_events,
    };
}

pub mod color {
    pub use sge_color::schemes::ColorScheme;
    pub use sge_color::u8::ColorU8;
    pub use sge_color::{COLOR_MAP, Color, Palette, str_to_color};
}

pub mod programs {
    pub use sge_programs::include_program;
    pub use sge_programs::load_program_sync;
}

pub mod camera {
    pub use sge_camera::{
        camera2d_zoom_at, cameras_for_resolution, d2::Camera2D, d3::Camera3D, get_camera_2d,
        get_camera_2d_mut, get_camera_3d, get_camera_3d_mut, get_flat_projection,
        screen_distance_to_world, screen_to_world, world_distance_to_screen, world_to_screen,
        world_to_screen_3d,
    };
    pub use sge_camera_controllers::first_person::FirstPersonCameraController;
    pub use sge_camera_controllers::orbit::OrbitCameraController;
    pub use sge_camera_controllers::pan::PanningCameraController;
    pub use sge_camera_controllers::shake::CameraShakeController;
}

pub mod time {
    pub use sge_time::{
        delta_time, frame_count, frames_since_input, is_first_frame, is_physics_time_paused,
        is_physics_time_paused_mut, once_per_n_seconds, once_per_second, oscillate, oscillate_t,
        pause_physics_timer, physics_delta_time, physics_speed, physics_speed_mut, physics_time,
        play_physics_timer, set_physics_speed, time, time_seconds, time_since,
        toggle_every_n_seconds, toggle_physics_timer,
    };
}

pub mod rng {
    pub use sge_rng::{
        const_random::const_random, get_next_counter, get_random, id, maybe_rand_choice, rand,
        rand_bool, rand_choice, rand_color, rand_f32, rand_range, rand_ratio, rand_usize,
        rand_vec2, rand_vec3, rand_vec4,
    };
}

pub mod utils {
    pub use sge_utils::{
        CapacityReached, ConstantArray, FromF32, Lerpable, Lerped, PartialClamp, RotatingArray,
        ToF32,
    };
}

pub mod shapes {
    pub use sge_api::shapes_2d::*;
    pub use sge_shapes::d2::*;
}

pub mod sdf {
    pub use sge_types::{Metaball, Metaballs};
    pub use sge_types::{Sdf, SdfFill, SdfShape, SdfStroke};
}

pub mod d3 {
    pub use sge_rendering::api::{
        create_blinn_phong_material, create_flat_material, create_gouraud_material,
        create_textured_material,
    };
    pub use sge_rendering::object_3d::{
        Mesh, MeshRef, Object3D, Object3DRef, ObjectLoadingError, ObjectToDraw,
    };
    pub use sge_rendering::shapes_3d::*;
    pub use sge_types::{MaterialVertex3D, Vertex3D};
}

pub mod textures {
    pub use atlas::*;
    pub mod atlas {
        pub use sge_texture_atlas::{
            Sprite, SpriteKey, TextureAtlas, TextureAtlasRef, create_spritesheet,
        };
    }
    pub use sge_macros::include_texture;
    pub use sge_rendering::api::{
        draw_fullscreen_texture, draw_nine_slice, draw_nine_slice_to, draw_nine_slice_world,
        draw_texture, draw_texture_ex, draw_texture_scaled, draw_texture_scaled_world,
        draw_texture_world, draw_texture_world_ex,
    };
    pub use sge_textures::{
        LoadTextureError, SgeTexture, TextureRef, load_texture_from_bytes_sync, load_texture_sync,
        num_registered_textures,
    };
    pub use sge_types::ResizeMethod;
}

pub mod render_textures {
    pub use sge_rendering::api::{
        create_empty_render_texture, end_rendering_to_texture, start_rendering_to_texture,
    };
    pub use sge_rendering::pipeline::RenderTextureRef;
}

pub mod logging {
    pub use log::{self, Level, LevelFilter, debug, error, info, trace, warn};
    pub use sge_logging::{
        Logger, draw_logs, log_lines, log_to_file, set_logger_verbosity, set_max_drawn_log_lines,
        set_min_log_level,
    };
    pub use sge_types::Verbosity;
}

pub mod rendering {
    pub use sge_rendering::d2::{Renderer2D, Scene2D};
    pub use sge_rendering::materials::{DEFAULT_MATERIAL, Material, MaterialRef, UniformData};

    pub use glium::{
        draw_parameters::PolygonMode,
        glutin::{
            config::{ColorBufferType, ConfigSurfaceTypes, ConfigTemplateBuilder},
            context::{ContextAttributesBuilder, GlProfile, Priority, ReleaseBehavior, Robustness},
            surface::{SurfaceAttributesBuilder, SwapInterval},
        },
        uniforms::{MagnifySamplerFilter, MinifySamplerFilter},
    };
    pub use sge_rendering::api::{
        clear_screen, dont_clear_screen, draw_scene, draw_scene_to, draw_scene_world,
    };
    pub use sge_rendering::pipeline::new_draw_queues;
    pub use sge_rendering::scissor::{
        clear_scissor_stack, current_scissor, get_scissor_stack, pop_scissor, push_scissor,
    };
    pub use sge_rendering::{take_screenshot, take_screenshot_image, window_texture};
    pub use sge_types::{BufferError, ColorVertex2D, Pattern, SpriteVertex, TexturedVertex2D};
}

pub mod animation {
    pub use sge_animation::*;
}

pub mod math {
    pub use sge_api::area::AreaExt;
    pub use sge_math::Vec2Ext;
    pub use sge_math::collision::{self, Aabb2d, IntersectsWith};
    pub use sge_math::curves::*;
    pub use sge_math::transform::{Transform2D, Transform3D};
    pub use sge_math::usize_rect::USizeRect;
    pub use sge_math::*;
    pub use sge_types::Area;
    pub use sge_vectors as extra_math_types;
    pub use sge_vectors::{
        IVec2, IVec3, IVec4, Mat2, Mat3, Mat4, Quat, USizeVec2, USizeVec3, USizeVec4, UVec2, UVec3,
        UVec4, Vec2, Vec2Swizzles, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles, VectorSpace, ivec2,
        ivec3, ivec4, mat2, mat3, mat4, ops::*, usizevec2, usizevec3, usizevec4, uvec2, uvec3,
        uvec4, vec2, vec3, vec4,
    };
    pub use std::f32::consts::*;
}

pub mod window {
    pub use glium::glutin::surface::WindowSurface;
    pub use glium::winit::dpi::{LogicalSize, PhysicalSize, Position, Size};
    pub use glium::winit::event::MouseButton;
    pub use glium::winit::keyboard::{Key, KeyCode, NamedKey};
    pub use glium::winit::monitor::MonitorHandle;
    pub use glium::winit::window::{
        Cursor, Fullscreen, Icon, Theme, WindowAttributes, WindowButtons, WindowLevel,
    };
    pub use sge_types::window_area;
    pub use sge_window::{
        IntoWindowSize, SgeDisplay, WindowCreationError, WindowOptions, WindowState,
        availible_monitors, cancel_window_request_user_attention, current_monitor, dpi_scaling,
        end_of_frame, fullscreen, get_display, get_display_mut, get_window_state, grab_cursor,
        is_fullscreen, is_window_decorated, is_window_focused, is_window_maximized,
        is_window_minimised, is_window_minimized, is_window_resizable, is_window_visible,
        max_window_dimension, min_window_dimension, opengl_context, opengl_version,
        primary_monitor, release_cursor, request_window_inner_size, resize_increments,
        set_cursor_grab, set_cursor_position, set_cursor_visible, set_fullscreen,
        set_max_window_inner_size, set_min_window_inner_size, set_window_blur,
        set_window_content_protected, set_window_cursor_hittest, set_window_decorations,
        set_window_icon, set_window_level, set_window_maximized, set_window_resizable,
        set_window_theme, set_window_title, set_window_visible, window_aspect_ratio, window_center,
        window_enabled_buttons, window_height, window_id, window_inner_position, window_inner_size,
        window_outer_position, window_outer_size, window_request_user_attention, window_size,
        window_size_u32, window_theme, window_title, window_width,
    };
}

pub mod cursor_icons {
    pub use sge_window::{
        use_alias_cursor_icon, use_all_resize_cursor_icon, use_all_scroll_cursor_icon,
        use_cell_cursor_icon, use_col_resize_cursor_icon, use_context_menu_cursor_icon,
        use_copy_cursor_icon, use_crosshair_cursor_icon, use_cursor_icon, use_default_cursor_icon,
        use_dnd_ask_cursor_icon, use_e_resize_cursor_icon, use_ew_resize_cursor_icon,
        use_grab_cursor_icon, use_grabbing_cursor_icon, use_help_cursor_icon, use_move_cursor_icon,
        use_n_resize_cursor_icon, use_ne_resize_cursor_icon, use_nesw_resize_cursor_icon,
        use_no_drop_cursor_icon, use_not_allowed_cursor_icon, use_ns_resize_cursor_icon,
        use_nw_resize_cursor_icon, use_nwse_resize_cursor_icon, use_pointer_cursor_icon,
        use_progress_cursor_icon, use_row_resize_cursor_icon, use_s_resize_cursor_icon,
        use_se_resize_cursor_icon, use_sw_resize_cursor_icon, use_text_cursor_icon,
        use_vertical_text_cursor_icon, use_w_resize_cursor_icon, use_wait_cursor_icon,
        use_zoom_in_cursor_icon, use_zoom_out_cursor_icon,
    };
}

#[cfg(feature = "audio")]
pub mod audio {
    pub use sge_audio::{
        Sound, SoundBuilder, SoundLoadError, SoundRef, include_sound, load_sound_from_bytes_sync,
        load_sound_sync, play_sound, play_sound_ex, rodio,
        rodio::BitDepth,
        rodio::source::{
            AutomaticGainControlSettings, LimitSettings, dither::Algorithm as DitherAlgorithm,
        },
    };
    pub use std::time::Duration;
}

#[cfg(feature = "gamepad")]
pub mod gamepad {
    pub use sge_input::gamepad_input as input;
    pub use sge_input::gilrs::*;
}

#[cfg(feature = "input")]
pub mod input {
    pub use crate::api::is_about_to_wait_for_input;
    pub use sge_input::{
        Action, Button, Input, action_held, action_pressed, action_pressed_os, action_released,
        bind, button_held, button_pressed, button_released, close_requested, cursor, cursor_diff,
        cursor_movements, cursor_prev, destroyed, dropped_file, gamepad::GamepadExt, get_all_binds,
        held_alt, held_control, held_shift, hovered_file, input_text, key_held, key_held_logical,
        key_pressed, key_pressed_logical, key_pressed_os, key_pressed_os_logical, key_released,
        key_released_logical, keys::KeyToString, last_cursor_pos, mouse_diff, mouse_held,
        mouse_pressed, mouse_released, pressed_movement_vector, resolution, scale_factor,
        scale_factor_changed, scroll_diff, should_quit, window_resized,
    };
    pub use sge_macros::{actions, bind};
}

#[cfg(feature = "text")]
pub mod text {
    pub use sge_exec::fs::{load_font, load_font_from_bytes};
    pub use sge_text::api::*;
    pub use sge_text::rich_text::*;
    pub use sge_text::{
        DEFAULT_FONT_SIZE, Font, FontRef, FontType, Glyph, LoadFontError, MONO, MONO_TYPEFACE,
        TextDimensions, TextDrawParams, TextDrawParamsBuilder, Typeface, create_ttf_font,
        default_font, icons, load_bitmap_font_sync, load_font_sync, wrapped_text,
    };
}

#[cfg(feature = "clipboard")]
/// utilities for interacting with the clipboard
pub mod clipboard {
    pub use sge_input::{get_clipboard_text, set_clipboard_text};
}

#[cfg(feature = "egui")]
pub mod egui_mod {
    pub use sge_egui::egui_glium::egui_winit::egui;
    pub use sge_egui::run_egui;
}

#[cfg(feature = "debugging")]
pub mod debugging {
    pub use sge_debugging::{
        avg_fps, get_draw_calls, get_drawn_objects, get_engine_time, get_index_count,
        get_max_draw_calls, get_max_drawn_objects, get_max_engine_time, get_max_index_count,
        get_max_vertex_count, get_vertex_count, max_fps, min_fps,
    };
}

#[cfg(feature = "extra_fonts")]
pub mod extra_fonts {
    pub use sge_text::{SANS, SANS_BOLD, SANS_BOLD_ITALIC, SANS_DISPLAY, SANS_TYPEFACE};
}

#[cfg(feature = "debug_visualisations")]
pub mod debug_visualisations {
    pub use sge_debug_visualisations::grid::{create_infinite_grid, draw_2d_grid_world};
    pub use sge_debug_visualisations::*;
}

pub mod fs {
    pub use sge_exec::fs::*;
}

pub mod exec {
    pub use sge_exec::{FrameFuture, futures::join, start_coroutine, wait_for, wait_for_frames};
}

#[cfg(feature = "multiplayer")]
pub mod multiplayer {
    pub use sge_multiplayer::{
        MultiplayerError, MultiplayerState, Notification, UserData,
        backends::{Message, MultiplayerBackend, itty::IttyBackend},
    };
}

#[cfg(feature = "network")]
pub mod net {
    pub use sge_exec::net::*;
}
