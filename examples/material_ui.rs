use sge::*;
use ui::*;

#[main("Material UI")]
fn main() {
    let mut v: f32 = rand();

    loop {
        if once_per_second() {
            v = rand();
        }

        if key_pressed(KeyCode::Space) {
            material::set_theme_dark(!material::get_theme_dark());
        }

        clear_screen(material::scheme().background);

        let ui = Padding::all(
            40.0,
            Col::with_gap(
                20.0,
                [
                    material::Text::headline_large("Material UI"),
                    material::ProgressBar::primary(300.0, v, 1.0, id!()),
                    material::Card::interactable(material::Text::on_primary_container(
                        "Material card",
                    ))
                    .min_width(300.0)
                    .fit(),
                    material::Card::outlined(material::Text::on_primary_container(
                        "Outlined material card",
                    ))
                    .min_width(300.0)
                    .fit(),
                    material::Card::elevated(material::Text::on_primary_container(
                        "Elevated material card",
                    ))
                    .min_width(300.0)
                    .fit(),
                    material::TextInput::surface(id!(), Some("Field".to_string()), 300.0),
                    material::LoadingSpinner::primary(),
                    material::Drawer::new(
                        id!(),
                        300.0,
                        "Drawer",
                        material::Text::on_surface("Hello world!"),
                    ),
                ],
            ),
        );

        draw_ui_in_area(ui, window_area());

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
