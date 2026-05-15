pub use crate::api::*;
pub use crate::{init, init_custom};
pub use anyhow;
pub use glium::winit::dpi::{LogicalSize, PhysicalSize};
pub use glium::winit::event::MouseButton;
pub use glium::winit::keyboard::{Key, KeyCode, NamedKey};
pub use glium::winit::monitor::MonitorHandle;
pub use glium::{
    draw_parameters::PolygonMode,
    glutin::{
        config::{ColorBufferType, ConfigSurfaceTypes, ConfigTemplateBuilder},
        context::{ContextAttributesBuilder, GlProfile, Priority, ReleaseBehavior, Robustness},
        surface::{SurfaceAttributesBuilder, SwapInterval, WindowSurface},
    },
    uniforms::{MagnifySamplerFilter, MinifySamplerFilter},
    winit::{
        dpi::{Position, Size},
        window::{Cursor, Fullscreen, Icon, Theme, WindowAttributes, WindowButtons, WindowLevel},
    },
};
pub use image::{self, ImageFormat};
pub use log::{self, Level, LevelFilter, debug, error, info, trace, warn};
pub use sge_animation::*;
pub use sge_api::area::AreaExt;
pub use sge_api::shapes_2d::*;
pub use sge_api::{Drawable, draw, draw_world};
pub use sge_camera::{
    camera2d_zoom_at, cameras_for_resolution, get_camera_2d, get_camera_2d_mut, get_camera_3d,
    get_camera_3d_mut, get_flat_projection, screen_distance_to_world, screen_to_world,
    world_distance_to_screen, world_to_screen, world_to_screen_3d,
};
pub use sge_camera_controllers::first_person::FirstPersonCameraController;
pub use sge_camera_controllers::orbit::OrbitCameraController;
pub use sge_camera_controllers::pan::PanningCameraController;
pub use sge_camera_controllers::shake::CameraShakeController;
pub use sge_color::schemes::ColorScheme;
pub use sge_color::{self as color, Color, Palette};
pub use sge_config::{
    EngineConfig, EngineCreationOptions, Opts, OptsBuilder, dont_wait_for_events, get_config,
    get_polygon_mode, get_wait_for_events, get_wait_for_events_mut, set_magnify_filter,
    set_minify_filter, set_polygon_mode, set_wait_for_events, toggle_wait_for_events,
    toggle_wireframe, use_default_filtering, use_linear_filtering, use_mipmaps,
    use_nearest_filtering, use_positive_y_down, use_positive_y_up, wait_for_events,
};
pub use sge_image::{Image, ImageRef, LoadImageError, load_image_sync};
pub use sge_include_assets::sge_include_assets;
pub use sge_logging::{
    Logger, draw_logs, log_lines, log_to_file, set_logger_verbosity, set_max_drawn_log_lines,
    set_min_log_level,
};
pub use sge_macros::include_texture;
pub use sge_macros::main;
pub use sge_macros::{actions, bind};
pub use sge_math::Vec2Ext;
pub use sge_math::collision::{self, Aabb2d, IntersectsWith};
pub use sge_math::transform::{Transform2D, Transform3D};
pub use sge_math::usize_rect::USizeRect;
pub use sge_particles::*;
pub use sge_persistence::{Error as PersistenceError, persistent, rkyv};
pub use sge_physics::{
    Bounds, ColliderConfig, CollisionPoints, ObjectRef, PhysicsWorld, WorldRef,
    player::PlayerBindBuilder, player::PlayerController,
};
pub use sge_programs::include_program;
pub use sge_programs::load_program_sync;
pub use sge_rendering::api::*;
pub use sge_rendering::d2::{Renderer2D, Scene2D};
pub use sge_rendering::materials::{DEFAULT_MATERIAL, Material, MaterialRef, UniformData};
pub use sge_rendering::object_3d::{
    Mesh, MeshRef, Object3D, Object3DRef, ObjectLoadingError, ObjectToDraw,
};
pub use sge_rendering::pipeline::RenderTextureRef;
pub use sge_rendering::pipeline::new_draw_queues;
pub use sge_rendering::post_processing::{
    PostProcessingEffect, PostProcessingError, RenderFullscreenQuadError,
    add_post_processing_effect, bloom_screen, blur_screen, brighten_screen,
    chromatic_abberation_screen, contrast_screen, film_grain_screen, greyscale_screen,
    hue_rotate_screen, invert_screen, pixelate_screen, saturate_screen, sharpen_screen,
    vignette_screen,
};
pub use sge_rendering::scissor::{
    clear_scissor_stack, current_scissor, get_scissor_stack, pop_scissor, push_scissor,
};
pub use sge_rendering::shapes_3d::*;
pub use sge_rendering::{take_screenshot, take_screenshot_image, window_texture};
pub use sge_rng::{
    const_random::const_random, get_next_counter, get_random, id, maybe_rand_choice, rand,
    rand_bool, rand_choice, rand_color, rand_f32, rand_range, rand_ratio, rand_usize, rand_vec2,
    rand_vec3, rand_vec4,
};
pub use sge_shapes::d2::*;
pub use sge_texture_atlas::{Sprite, SpriteKey, TextureAtlas, TextureAtlasRef, create_spritesheet};
pub use sge_textures::{
    LoadTextureError, SgeTexture, TextureRef, load_texture_from_bytes_sync, load_texture_sync,
    num_registered_textures,
};
pub use sge_time::{
    delta_time, frame_count, frames_since_input, is_first_frame, is_physics_time_paused,
    is_physics_time_paused_mut, once_per_n_seconds, once_per_second, oscillate, oscillate_t,
    pause_physics_timer, physics_delta_time, physics_speed, physics_speed_mut, physics_time,
    play_physics_timer, set_physics_speed, time, time_seconds, time_since, toggle_every_n_seconds,
    toggle_physics_timer,
};
pub use sge_types::{
    Area, BufferError, ColorVertex2D, MaterialVertex3D, Pattern, Sdf, SdfFill, SdfShape, SdfStroke,
    SpriteVertex, TexturedVertex2D, Verbosity, Vertex3D, window_area,
};
pub use sge_types::{Metaball, Metaballs};
pub use sge_utils::RotatingArray;
pub use sge_vectors::{
    IVec2, IVec3, IVec4, Mat2, Mat3, Mat4, Quat, USizeVec2, USizeVec3, USizeVec4, UVec2, UVec3,
    UVec4, Vec2, Vec2Swizzles, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles, VectorSpace, ivec2, ivec3,
    ivec4, mat2, mat3, mat4, ops::*, usizevec2, usizevec3, usizevec4, uvec2, uvec3, uvec4, vec2,
    vec3, vec4,
};
pub use sge_window::{
    IntoWindowSize, SgeDisplay, WindowCreationError, WindowOptions, WindowState,
    availible_monitors, cancel_window_request_user_attention, current_monitor, dpi_scaling,
    end_of_frame, fullscreen, get_display, get_display_mut, get_window_state, grab_cursor,
    is_fullscreen, is_window_decorated, is_window_focused, is_window_maximized,
    is_window_minimised, is_window_minimized, is_window_resizable, is_window_visible,
    max_window_dimension, min_window_dimension, opengl_context, opengl_version, primary_monitor,
    release_cursor, request_window_inner_size, resize_increments, set_cursor_grab,
    set_cursor_position, set_cursor_visible, set_fullscreen, set_max_window_inner_size,
    set_min_window_inner_size, set_window_blur, set_window_content_protected,
    set_window_cursor_hittest, set_window_decorations, set_window_icon, set_window_level,
    set_window_maximized, set_window_resizable, set_window_theme, set_window_title,
    set_window_visible, use_alias_cursor_icon, use_all_resize_cursor_icon,
    use_all_scroll_cursor_icon, use_cell_cursor_icon, use_col_resize_cursor_icon,
    use_context_menu_cursor_icon, use_copy_cursor_icon, use_crosshair_cursor_icon, use_cursor_icon,
    use_default_cursor_icon, use_dnd_ask_cursor_icon, use_e_resize_cursor_icon,
    use_ew_resize_cursor_icon, use_grab_cursor_icon, use_grabbing_cursor_icon,
    use_help_cursor_icon, use_move_cursor_icon, use_n_resize_cursor_icon,
    use_ne_resize_cursor_icon, use_nesw_resize_cursor_icon, use_no_drop_cursor_icon,
    use_not_allowed_cursor_icon, use_ns_resize_cursor_icon, use_nw_resize_cursor_icon,
    use_nwse_resize_cursor_icon, use_pointer_cursor_icon, use_progress_cursor_icon,
    use_row_resize_cursor_icon, use_s_resize_cursor_icon, use_se_resize_cursor_icon,
    use_sw_resize_cursor_icon, use_text_cursor_icon, use_vertical_text_cursor_icon,
    use_w_resize_cursor_icon, use_wait_cursor_icon, use_zoom_in_cursor_icon,
    use_zoom_out_cursor_icon, window_aspect_ratio, window_center, window_enabled_buttons,
    window_height, window_id, window_inner_position, window_inner_size, window_outer_position,
    window_outer_size, window_request_user_attention, window_size, window_size_u32, window_theme,
    window_title, window_width,
};
pub mod graph_networks {
    pub use sge_graph_networks::*;
}
pub use anyhow::Result as AResult;

#[cfg(feature = "audio")]
pub use audio::*;
#[cfg(feature = "audio")]
mod audio {
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

pub mod math {
    pub use sge_math::*;
    pub use sge_vectors::*;
}

#[cfg(feature = "input")]
pub use input::*;
#[cfg(feature = "input")]
pub mod input {
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
}

#[cfg(feature = "clipboard")]
pub use clipboard::*;
#[cfg(feature = "clipboard")]
pub mod clipboard {
    pub use sge_input::{get_clipboard_text, set_clipboard_text};
}

#[cfg(feature = "text")]
pub use text::*;
#[cfg(feature = "text")]
mod text {
    pub use sge_text::rich_text::*;
    pub use sge_text::{
        FontRef, Glyph, LoadFontError, MONO, SANS_ITALIC, SgeFont, TextDimensions, TextDrawParams,
        TextMeasureCache, create_ttf_font, default_font, draw_colored_text,
        draw_colored_text_world, draw_multiline_text, draw_multiline_text_ex,
        draw_multiline_text_size, draw_multiline_text_size_world, draw_multiline_text_world,
        draw_multiline_text_world_ex, draw_text, draw_text_custom, draw_text_ex, draw_text_size,
        draw_text_size_world, draw_text_world, draw_text_world_custom, draw_text_world_ex,
        draw_wrapped_text_in_area, icons, load_font_sync, measure_multiline_text,
        measure_multiline_text_ex, measure_text, measure_text_ex, measure_wrapped_text,
        wrap_text_to_width, wrapped_text,
    };
}

#[cfg(feature = "extra_fonts")]
pub use extra_fonts::*;
#[cfg(feature = "extra_fonts")]
mod extra_fonts {
    pub use sge_text::{SANS, SANS_BOLD, SANS_BOLD_ITALIC, SANS_DISPLAY};
}

#[cfg(feature = "ui")]
pub use sge_ui::prelude as ui;

#[cfg(feature = "egui")]
pub use egui_mod::*;
#[cfg(feature = "egui")]
mod egui_mod {
    pub use sge_egui::egui_glium::egui_winit::egui;
    pub use sge_egui::run_egui;
}

#[cfg(feature = "debugging")]
pub use debugging::*;
#[cfg(feature = "debugging")]
mod debugging {
    pub use sge_debugging::{
        avg_fps, get_draw_calls, get_drawn_objects, get_engine_time, get_index_count,
        get_max_draw_calls, get_max_drawn_objects, get_max_engine_time, get_max_index_count,
        get_max_vertex_count, get_vertex_count, max_fps, min_fps,
    };
}

#[cfg(feature = "debug_visualisations")]
pub use debug_visualisations::*;
#[cfg(feature = "debug_visualisations")]
mod debug_visualisations {
    pub use sge_debug_visualisations::grid::{create_infinite_grid, draw_2d_grid_world};
    pub use sge_debug_visualisations::*;
}

pub use crate::{next_frame, run_async};
pub use sge_exec::{FrameFuture, fs::*, futures::join, start_coroutine, wait_for, wait_for_frames};

#[cfg(feature = "network")]
pub use sge_exec::net;
