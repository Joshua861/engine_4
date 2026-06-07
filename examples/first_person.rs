use sge::{
    prelude::*,
};
use ui::*;

#[main("First person")]
async fn main() -> anyhow::Result<()> {
    let camera = get_camera_3d_mut();
    camera.set_fovy(120.0);
    camera.translate_by(vec3(0.0, 0.0, 0.0));

    let mut controller = FirstPersonCameraController::new(0.002);

    let light_pos = Vec3::new(2.0, 5.0, 0.0);
    let material = create_gouraud_material(Color::SLATE_300, Color::SLATE_500, light_pos);
    let data = include_bytes!("../assets/models/suzanne.obj");
    let suzanne = Object3D::from_obj_bytes_with_material(data, material)?
        .with_transform(Transform3D::from_translation(vec3(0.0, 0.0, -5.0)));
    let material = create_flat_material(Color::NEUTRAL_800.blend_halfway(Color::NEUTRAL_700));
    let cuboid =
        Object3D::from_mesh_and_material(cuboid(Vec3::ZERO, vec3(1.0, 1.0, 1.0)), material);

    let transforms: Vec<_> = (0..50)
        .map(|_| {
            Transform3D::from_translation(vec3(
                rand::<f32>() * 20.0 - 10.0,
                rand::<f32>() * 20.0 - 10.0,
                rand::<f32>() * 20.0 - 10.0,
            ))
        })
        .collect();

    loop {
        if mouse_pressed(MouseButton::Left) {
            controller.enable();
        }

        if key_pressed(KeyCode::Escape) {
            controller.disable();
        }

        clear_screen(Color::NEUTRAL_800);
        controller.update();

        cuboid.draw_many(transforms.clone());
        suzanne.draw();

        if let Some(screen_pos) = world_to_screen_3d(vec3(0.0, 1.0, -5.0)) {
            draw_ui_in_area(
                Center::new(
                    Text::new_with_color("Suzanne", Color::WHITE)
                        .padding(10.0)
                        .fill(Color::BLACK)
                        .fit(),
                ),
                Area::new(screen_pos - Vec2::splat(50.0), Vec2::splat(100.0)),
            );
        }

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}
