use sge::prelude::*;

const PLAYER_RADIUS: f32 = 30.0;

#[main("2D Platformer")]
fn main() -> anyhow::Result<()> {
    let mut world = PhysicsWorld::new();

    let mut player = world
        .create_player_controller(Bounds::Circle(PLAYER_RADIUS))
        .with_position(Vec2::new(0.0, -200.0));

    player
        .set_binds()
        .jump(KeyCode::Space)
        .right(KeyCode::KeyF)
        .left(KeyCode::KeyS);

    player.set_move_speed(5.0);
    player.set_double_jumps(1);
    player.set_jump_velocity(13.0);
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

    loop {
        clear_screen(Color::NEUTRAL_900);

        if key_pressed(KeyCode::KeyR) {
            player.set_position(Vec2::new(0.0, -200.0));
        }

        if player.position().y > 1000.0 {
            player.add_impulse(Vec2::Y * 15.0);
        }

        draw_circle_world(player.position(), PLAYER_RADIUS, Color::RED_500);

        for platform in &platforms {
            draw_world(platform);
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
