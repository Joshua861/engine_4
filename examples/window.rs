use sge::prelude::*;

#[main("Window")]
fn main() {
    loop {
        let window_info = format!(
            "=== Window Information ===\n\
     Title:           {}\n\
     ID:              {:?}\n\
     \n\
     --- Size & Position ---\n\
     Size:            {}x{} ({}x{} u32)\n\
     Inner Size:      {:?}\n\
     Outer Size:      {:?}\n\
     Inner Position:  {:?}\n\
     Outer Position:  {:?}\n\
     Center:          {:?}\n\
     Min Dimension:   {:?}\n\
     Max Dimension:   {:?}\n\
     Aspect Ratio:    {:?}\n\
     Resize Incr:     {:?}\n\
     \n\
     --- Display & DPI ---\n\
     DPI Scaling:     {}\n\
     Primary Monitor: {:?}\n\
     Current Monitor: {:#?}\n\
     OpenGL Version:  {:?}\n\
     \n\
     --- State ---\n\
     Visible:         {:?}\n\
     Focused:         {}\n\
     Maximized:       {}\n\
     Minimized:       {:?}\n\
     Fullscreen:      {}\n\
     Decorated:       {}\n\
     Resizable:       {}\n\
     Theme:           {:?}\n\
     Enabled Buttons: {:?}\n\
    ",
            window_title(),
            window_id(),
            // Size
            window_width(),
            window_height(),
            window_size_u32().x,
            window_size_u32().y,
            window_inner_size(),
            window_outer_size(),
            window_inner_position(),
            window_outer_position(),
            window_center(),
            min_window_dimension(),
            max_window_dimension(),
            window_aspect_ratio(),
            resize_increments(),
            // Display
            dpi_scaling(),
            primary_monitor(),
            current_monitor(),
            opengl_version(),
            // State
            is_window_visible(),
            is_window_focused(),
            is_window_maximized(),
            is_window_minimized(),
            is_fullscreen(),
            is_window_decorated(),
            is_window_resizable(),
            window_theme(),
            window_enabled_buttons(),
        );

        clear_screen(Color::WHITE);

        draw_multiline_text_ex(
            window_info,
            TextDrawParams::builder()
                .color(Color::BLACK)
                .position(vec2(20.0, 20.0))
                .build(),
            1.0,
        );

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
