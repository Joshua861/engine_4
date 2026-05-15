use std::f32::consts::{FRAC_PI_2, PI};

use sge::prelude::*;

const PLAYER_RADIUS: f32 = 30.0;

struct Player {
    color: Color,
    controller: PlayerController,
}

#[main("2D Platformer")]
async fn main() -> anyhow::Result<()> {
    let mut world = PhysicsWorld::new();

    let mut player = Player {
        color: Color::RED_500,
        controller: world
            .create_player_controller(Bounds::Rect(Vec2::splat(PLAYER_RADIUS * 2.0)))
            .with_position(Vec2::new(0.0, -200.0)),
    };

    const PREV_LEN: usize = 100;
    const PREV_REPEAT: usize = 10;
    let mut prev_pos = RotatingArray::new([Vec2::ZERO; PREV_LEN]);

    player
        .controller
        .set_binds()
        .jump(KeyCode::Space)
        .right(KeyCode::KeyF)
        .left(KeyCode::KeyS);

    player.controller.set_move_speed(5.0);
    player.controller.set_double_jumps(1);
    player.controller.set_jump_velocity(13.0);
    world.set_gravity(50.0);

    let platforms = vec![
        platform(-500.0, 400.0, 1000.0),
        platform(-400.0, 250.0, 200.0),
        platform(200.0, 250.0, 200.0),
        platform(-100.0, 100.0, 200.0),
    ];

    for p in &platforms {
        world
            .create_fixed(Bounds::Rect(p.size))
            .with_position(p.center());
    }

    let mut ps = ParticleSystem::new();

    let jump_particles = ParticleOneshot::builder()
        .shape(&Rect::new_square(
            Vec2::ZERO,
            10.0,
            Color::WHITE.with_alpha(0.3),
        ))
        .color_randomness(Color::BLACK.with_alpha(0.1))
        .end_color(Color::WHITE.with_alpha(0.0))
        .speed(5.0)
        .direction(-PI)
        .direction_randomness(FRAC_PI_2)
        .speed_randomness(3.0)
        .lifetime_randomness(0.5)
        .rotation_speed_randomness(0.2)
        .lifetime(1.0)
        .build();

    let land_particles = ParticleOneshot::builder()
        .shape(&Rect::new_square(
            Vec2::ZERO,
            10.0,
            Color::WHITE.with_alpha(0.1),
        ))
        .color_randomness(Color::BLACK.with_alpha(0.1))
        .end_color(Color::WHITE.with_alpha(0.0))
        .speed(5.0)
        .direction_randomness(FRAC_PI_2)
        .acceleration(vec2(0.0, 0.2))
        .speed_randomness(3.0)
        .rotation_speed_randomness(0.2)
        .quantity(5)
        .quantity_randomness(2.0)
        .lifetime_randomness(0.5)
        .lifetime(0.5)
        .build();

    let mut debug_mode = false;

    let mut camera_controller = PanningCameraController {
        allow_panning: false,
        ..Default::default()
    };

    loop {
        ps.update();
        camera_controller.update();

        if !debug_mode {
            clear_screen(Color::NEUTRAL_900);
            vignette_screen(Color::NEUTRAL_950, 0.5);
        }

        draw_simple_debug_info();

        draw_multiline_text(
            "Press D to toggle debug mode\nSpace to jump\nA to move right\nF to move left",
            Vec2::splat(10.0),
            1.0,
        );

        if !debug_mode {
            draw_2d_grid_world(Color::NEUTRAL_800);
        }

        if key_pressed(KeyCode::KeyD) {
            debug_mode = !debug_mode;
        }

        if key_pressed(KeyCode::KeyR) {
            player.controller.set_position(Vec2::new(0.0, -200.0));
        }

        if debug_mode {
            world.draw_colliders_world();
        }

        if player.controller.position().y > 1000.0
            && player.controller.velocity().y.is_sign_negative()
        {
            player.controller.add_impulse(Vec2::Y * 30.0);
        }

        if player.controller.just_double_jumped() {
            ps.spawn_oneshot(
                &jump_particles,
                player.controller.position() + vec2(0.0, PLAYER_RADIUS),
            );
        }

        if player.controller.just_landed() {
            ps.spawn_oneshot(
                &land_particles,
                player.controller.position() + vec2(0.0, PLAYER_RADIUS),
            );
        }

        {
            let ppos = player.controller.position();
            let cpos = get_camera_2d().translation();
            let (normal, len) = (ppos - cpos).normalize_and_length();

            if len > 300.0 {
                let pos = -normal * 300.0 + ppos;
                get_camera_2d_mut().set_translation(pos);
            }
        }

        for i in 0..PREV_REPEAT {
            let ratio = (i + 1) as f32 / PREV_REPEAT as f32;
            let pos = player
                .controller
                .position_last_frame()
                .lerp(player.controller.position(), ratio);
            prev_pos.push(pos);
        }

        if !debug_mode {
            for (i, pos) in prev_pos.iter().enumerate() {
                let age = i as f32 / PREV_LEN as f32;
                draw_circle_world(
                    *pos,
                    PLAYER_RADIUS * age,
                    player.color.darken(0.2).desaturate(0.5),
                );
            }

            draw_circle_world(player.controller.position(), PLAYER_RADIUS, player.color);

            for p in &platforms {
                draw_world(p);
            }

            ps.draw_world();
        }

        world.update();

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}

fn platform(x: f32, y: f32, w: f32) -> Rect {
    Rect::new(Vec2::new(x, y), Vec2::new(w, 20.0), Color::NEUTRAL_800)
}
