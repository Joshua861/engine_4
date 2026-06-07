use core::f32;
use std::f32::consts::PI;

use sge::prelude::icons::{ICON_BOMB, ICON_CARET_DOWN, ICON_CIRCLE_NOTCH, ICON_PLUS_CIRCLE};
use sge::prelude::*;
use ui::*;

const NODES: &[(&str, fn() -> UiRef)] = &[
    ("Align", align),
    ("Aspect Ratio", aspect_ratio),
    ("Border", border),
    ("Button", button),
    ("Center", center),
    ("Chart/line", line_chart),
    ("Circle", circle),
    ("Color selector", color_selector),
    ("Dock", dock),
    ("Drawer", drawer),
    ("Drop file", drop_file),
    ("Fill/active", active_fill),
    ("Fill/box", box_fill),
    ("Fill/fill", fill),
    ("Fill/gradient", gradient_fill),
    ("Fill/multigradient", multipoint_gradient_fill),
    ("Fill/pattern", pattern_fill),
    ("Fill/rounded", rounded_fill),
    ("Flow", flow),
    ("Hoverable", hoverable),
    ("Hyperlink", hyperlink),
    ("Image/async", async_image),
    ("Image/sync", image),
    ("Inactive Overlay", inactive_overlay),
    ("Input/data", data_input),
    ("Input/text", input),
    ("Layout/col", col),
    ("Layout/flex", flex),
    ("Layout/grid", grid),
    ("Layout/row", row),
    ("Loading Bar", loading_bar),
    ("Loading Spinner", loading_spinner),
    ("Modal", modal),
    ("Nine slice", nine_slice),
    ("Outline", outline),
    ("Progress Bar", progress_bar),
    ("Scissor Box", scissor_box),
    ("Scroll", scroll),
    ("Search", search),
    ("Selectbox", selectbox),
    ("Sized Box", sized_box),
    ("Slider", slider),
    ("Stack", stack),
    ("Text/rich", rich_text_page),
    ("Text", text_showcase),
    ("Tooltip", tooltip),
    ("Window", window),
];
const SCHEME: ColorScheme = ColorScheme::LACKLUSTER;

#[main("UI Showcase")]
async fn main() -> anyhow::Result<()> {
    let select = id!();

    loop {
        clear_screen(SCHEME.bg1);

        let selected = select_box_value(select).unwrap_or(0);

        let ui = SizedBox::new(
            window_size(),
            FlexRow::new([
                FlexBox::Fixed(
                    SizedBox::wh(
                        200.0,
                        window_height(),
                        flat::SelectBox::mono_text(
                            SCHEME.bg1,
                            SCHEME.bg2,
                            select,
                            NODES.iter().map(|n| n.0),
                        )
                        .scroll(id!()),
                    )
                    .scroll(id!())
                    .fill(SCHEME.bg1),
                ),
                FlexBox::Flex(BoxFill::new(
                    SCHEME.bg0,
                    Padding::all(
                        20.0,
                        FlexCol::new([
                            FlexBox::Flex(Col::with_gap(
                                40.0,
                                [
                                    Text::builder()
                                        .text(NODES[selected].0)
                                        .font(SANS_DISPLAY)
                                        .color(SCHEME.fg0)
                                        .font_size(40)
                                        .build(),
                                    (NODES[selected].1)(),
                                ],
                            )),
                            FlexBox::Fixed(Text::mono(avg_fps() as usize)),
                        ]),
                    ),
                )),
            ]),
        );

        draw_ui(ui, Vec2::ZERO);

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}

fn center() -> UiRef {
    Center::new(text("Hello from the center of the panel"))
}

fn aspect_ratio() -> UiRef {
    Col::with_gap(
        20.0,
        [
            AspectRatio::new(
                2.0 / 1.0,
                BoxFill::new(SCHEME.blue, Center::new(text("Hello world"))),
            )
            .max_width(400.0),
            AspectRatio::new(
                1.0 / 2.0,
                BoxFill::new(SCHEME.red, Center::new(text("Hello world"))),
            )
            .max_height(400.0),
        ],
    )
}

fn active_fill() -> UiRef {
    SizedBox::new(
        Vec2::splat(300.0),
        Fill::active(
            SCHEME.bg1,
            SCHEME.bg2,
            SCHEME.bg3,
            50.0,
            Center::new(text("Try hovering or clicking")),
        ),
    )
}

fn align() -> UiRef {
    fn container(x: AlignType, y: AlignType) -> UiRef {
        Fill::rounded(
            SCHEME.bg1,
            20.0,
            Align::new(
                x,
                y,
                Fill::rounded(
                    SCHEME.bg2,
                    10.0,
                    Center::new(Text::mono(format!("{}\n{}", x, y))),
                )
                .square(100.0),
            )
            .padding(10.0),
        )
        .square(200.0)
    }

    Fit::new(Grid::with_gap(
        3,
        3,
        20.0,
        [
            container(AlignType::Start, AlignType::Start),
            container(AlignType::Center, AlignType::Start),
            container(AlignType::End, AlignType::Start),
            container(AlignType::Start, AlignType::Center),
            container(AlignType::Center, AlignType::Center),
            container(AlignType::End, AlignType::Center),
            container(AlignType::Start, AlignType::End),
            container(AlignType::Center, AlignType::End),
            container(AlignType::End, AlignType::End),
        ],
    ))
}

fn border() -> UiRef {
    let a = BorderStyle::new(20.0, SCHEME.bg2);
    let b = BorderStyle::new(20.0, SCHEME.bg3);
    let c = BorderStyle::custom(20.0, SCHEME.bg2, BorderType::Rectangle);
    let d = BorderStyle::custom(20.0, SCHEME.bg3, BorderType::Rectangle);

    Col::with_gap(
        20.0,
        [
            Border::tblr(b, b, a, a, BoxFill::new(SCHEME.bg1, EMPTY)).square(200.0),
            Border::tblr(c, c, d, d, BoxFill::new(SCHEME.bg1, EMPTY)).square(200.0),
            Border::all_style(
                BorderStyle::custom(10.0, SCHEME.bg3, BorderType::Dashed(30.0)),
                EMPTY,
            )
            .square(200.0),
        ],
    )
}

fn box_fill() -> UiRef {
    BoxFill::new(SCHEME.green, EMPTY).square(200.0)
}

fn nine_slice() -> UiRef {
    struct State {
        texture: TextureRef,
    }

    if !storage_exists::<State>() {
        let texture = load_texture_sync("assets/textures/nine_slice_example.png").unwrap();
        storage_store_state(State { texture });
    }

    let texture = storage_get_state::<State>().texture;

    NineSliceFill::new(
        texture,
        Vec2::splat(10.0),
        3,
        ResizeMethod::Tile,
        EMPTY.fill(Color::BLACK).square(20.0),
    )
    .sized_wh(400.0, 300.0)
}

fn pattern_fill() -> UiRef {
    Col::with_gap(
        20.0,
        [
            Fill::builder()
                .color(Color::YELLOW_500)
                .alt_color(Color::TRANSPARENT)
                .pattern(SdfFill::Lines)
                .pattern_scale(1.0)
                .pattern_angle(0.75 * PI)
                .child(EMPTY)
                .build()
                .border(Color::YELLOW_500, 10.0)
                .sized_wh(600.0, 100.0),
            Fill::builder()
                .color(Color::NEUTRAL_100)
                .alt_color(Color::PINK_100)
                .pattern(SdfFill::Dots)
                .pattern_scale(4.0)
                .child(Text::bold_with_color("Hello!!! <3", Color::BLACK))
                .build()
                .border(Color::WHITE, 10.0)
                .square(300.0),
        ],
    )
}

fn button() -> UiRef {
    #[derive(Default)]
    struct ButtonExampleState {
        enabled: bool,
        counter: usize,
    }

    storage_init_state(ButtonExampleState::default());

    let state = storage_get_state_mut::<ButtonExampleState>();
    let button_a = id!();
    let button_b = id!();

    if button_clicked_last_frame(button_a) {
        state.enabled = !state.enabled;
    }

    if button_clicked_last_frame(button_b) {
        state.counter += 1;
    }

    Col::with_gap(
        20.0,
        [
            flat::Button::primary(button_a, Text::nowrap("Click me!")),
            if state.enabled {
                text("Thanks for clicking!")
            } else {
                EMPTY
            },
            flat::Button::primary_text(button_b, "Increment"),
            text(state.counter),
            flat::Button::danger(id!(), Text::nowrap("Delete account")),
        ],
    )
}

fn line_chart() -> UiRef {
    let data: Vec<_> = (0..90)
        .map(|i| sin((i as f32 * 5.0).to_radians()) + 1.0)
        .collect();

    Col::with_gap(
        20.0,
        [
            flat::LineChart::new(data, 500.0, 200.0),
            text(
                "This sin graph will look weird with the round_coords feature enabled, but that feature makes other parts of the UI look more crisp. You can enable and disable it in your Cargo.toml.",
            ),
            flat::Hyperlink::new("https://doc.rust-lang.org/cargo/reference/features.html"),
        ],
    )
}

fn circle() -> UiRef {
    CircleFill::new(SCHEME.fg3).sized_wh(200.0, 200.0)
}

fn col() -> UiRef {
    fn b(n: usize) -> UiRef {
        BoxFill::new(SCHEME.bg2, Center::new(text(n))).square(100.0)
    }

    Col::with_gap(20.0, [b(0), b(1), b(2), b(3), b(4)])
        .padding(20.0)
        .fill(SCHEME.bg1)
        .fit()
}

fn drawer() -> UiRef {
    flat::Drawer::new(
        "Open drawer...",
        SCHEME.bg1,
        id!(),
        text("Porro voluptate quis voluptatum voluptatem aspernatur et. Natus quos debitis repellendus voluptas voluptatem sit odit. Tenetur voluptatem quis quibusdam. Iusto et repellat et et eos consequuntur. Et nesciunt distinctio eveniet et et velit nihil.")
            .square(400.0)
    )
    .max_width(400.0)
}

fn fill() -> UiRef {
    let value: &'static mut f32 = State::from_id(id!()).get_or_default();

    Col::with_gap(
        20.0,
        [
            Fill::rounded(SCHEME.fg0, *value, EMPTY).square(200.0),
            flat::Slider::alternate(value, 0.0, 100.0, id!()).max_width(200.0),
            text("Draws a box fill if radius is 0, and a rounded fill otherwise"),
        ],
    )
}

fn gradient_fill() -> UiRef {
    Grid::new(
        2,
        2,
        [
            Fill::gradient(SCHEME.bg3, SCHEME.bg0, 0.0, EMPTY),
            Fill::gradient(SCHEME.bg3, SCHEME.bg0, PI * 0.5, EMPTY),
            Fill::gradient(SCHEME.bg3, SCHEME.bg0, PI, EMPTY),
            Fill::gradient(SCHEME.bg3, SCHEME.bg0, PI * 1.5, EMPTY),
        ],
    )
    .square(400.0)
}

fn flex() -> UiRef {
    SizedBox::wh(
        400.0,
        oscillate(200.0, 800.0),
        FlexCol::new([
            FlexBox::Fixed(SizedBox::wh(
                f32::INFINITY,
                100.0,
                Fill::new(SCHEME.fg2, EMPTY),
            )),
            FlexBox::Flex(Fill::new(SCHEME.bg2, DebugNode::new()).scissored()),
            FlexBox::Flex(Fill::new(SCHEME.bg3, EMPTY).scissored()),
            FlexBox::Fixed(SizedBox::wh(
                f32::INFINITY,
                100.0,
                Fill::new(SCHEME.fg2, EMPTY),
            )),
        ]),
    )
}

fn row() -> UiRef {
    fn b(n: usize) -> UiRef {
        BoxFill::new(SCHEME.bg2, Center::new(text(n))).square(100.0)
    }

    Row::with_gap(20.0, [b(0), b(1), b(2), b(3), b(4)])
        .padding(20.0)
        .fill(SCHEME.bg1)
        .fit()
}

fn grid() -> UiRef {
    fn b(n: usize) -> UiRef {
        BoxFill::new(SCHEME.bg2, Center::new(text(n))).square(100.0)
    }

    Grid::with_gap(
        3,
        3,
        20.0,
        [b(0), b(1), b(2), b(3), b(4), b(5), b(6), b(7), b(8)],
    )
    .padding(20.0)
    .fill(SCHEME.bg1)
    .fit()
}

fn hoverable() -> UiRef {
    let id = id!();

    Col::with_gap(
        20.0,
        [
            Hoverable::new(id, EMPTY).fill(SCHEME.bg2).square(200.0),
            if hovered_last_frame(id) {
                Text::body("Hovered")
            } else {
                EMPTY
            },
        ],
    )
}

fn color_selector() -> UiRef {
    flat::ColorSelector::new(id!(), Color::RED_500)
}

fn image() -> UiRef {
    struct State {
        image: TextureRef,
    }

    if !storage_exists::<State>() {
        let texture = load_texture_sync("assets/textures/guy.jpg").unwrap();
        storage_store_state(State { image: texture });
    }

    let image = storage_get_state::<State>().image;

    ImageNode::from_texture(image)
}

fn async_image() -> UiRef {
    struct State {
        image: LoadingTexture,
        other_image: LoadingTexture,
    }

    if !storage_exists::<State>() {
        storage_store_state(State {
            image: None,
            other_image: None,
        });

        start_coroutine(async {
            let state = storage_get_state_mut::<State>();

            state.other_image = Some(load_texture("assets/textures/i-dont-exist.jpg").await);
            state.image = Some(load_texture("assets/textures/space.jpg").await);
        });
    }

    let state = storage_get_state::<State>();

    Col::with_gap(
        20.0,
        [
            flat::AsyncImageNode::new(&state.image, vec2(600.0, 400.0)),
            flat::AsyncImageNode::new(&state.other_image, vec2(600.0, 400.0)),
        ],
    )
}

fn inactive_overlay() -> UiRef {
    InactiveOverlay::new(Color::BLACK.with_alpha(0.5), image()).fit()
}

fn input() -> UiRef {
    let input = id!();
    let value = text_input_value(input);

    Col::with_gap(
        20.0,
        [
            flat::TextInput::with_prompt(SCHEME.bg2, "Enter text...", input).max_width(300.0),
            Text::title(value),
        ],
    )
}

fn data_input() -> UiRef {
    use std::net::Ipv4Addr;

    let float_input = id!();
    let float_state = data_input_state::<f32>(float_input);

    let ip_input = id!();
    let ip_state = data_input_state::<Ipv4Addr>(ip_input);

    Col::with_gap(
        20.0,
        [
            flat::DataInput::<f32>::mono_with_prompt(SCHEME.bg1, "Enter float", float_input),
            Text::mono_nowrap(format!("{:#?}", float_state)),
            flat::DataInput::<Ipv4Addr>::mono_with_prompt(SCHEME.bg1, "Enter IPV4", ip_input),
            Text::mono_nowrap(format!("{:#?}", ip_state)),
            Text::mono_nowrap(format!(
                "is_private: {}",
                ip_state.value.map(|ip| ip.is_private()).unwrap_or(false)
            )),
        ],
    )
}

fn loading_bar() -> UiRef {
    LoadingBar::new(Color::SKY_400, Color::SKY_400.darken(0.05)).sized_wh(300.0, 30.0)
}

fn loading_spinner() -> UiRef {
    LoadingSpinner::new(SCHEME.fg1).square(100.0)
}

fn progress_bar() -> UiRef {
    struct LoadingBarExampleState {
        progress: usize,
    }

    if !storage_exists::<LoadingBarExampleState>() {
        storage_store_state(LoadingBarExampleState { progress: 0 });
    }

    let state = storage_get_state_mut::<LoadingBarExampleState>();

    let button = id!();

    if button_clicked_last_frame(button) {
        state.progress += 1;
    }

    if state.progress > 5 {
        state.progress = 0;
    }

    Col::with_gap(
        20.0,
        [
            ProgressBar::new(
                state.progress as f32,
                5.0,
                Fill::rounded(SCHEME.fg1, 10.0, EMPTY),
                EMPTY,
                id!(),
            )
            .padding(5.0)
            .rounded_fill(SCHEME.bg3, 15.0)
            .sized_wh(300.0, 50.0),
            flat::Button::primary_text(button, "Increment"),
        ],
    )
}

fn rounded_fill() -> UiRef {
    let value: &'static mut f32 = State::from_id(id!()).get_or_default();

    Col::with_gap(
        20.0,
        [
            Fill::rounded(SCHEME.fg0, *value, EMPTY).square(200.0),
            flat::Slider::alternate(value, 0.0, 100.0, id!()).max_width(200.0),
        ],
    )
}

fn scissor_box() -> UiRef {
    Col::new([
        ScissorBox::new(Text::title_nowrap("With Scissor Box")).max_width(350.0),
        Text::title_nowrap("Without Scissor Box").max_width(350.0),
    ])
    .fill(SCHEME.bg3)
    .fit()
}

fn scroll() -> UiRef {
    let items = (0..100)
        .map(|n| {
            SizedBox::height(
                40.0,
                Fill::rounded_hover(SCHEME.bg2, SCHEME.bg3, 7.0, Center::new(text(n))),
            )
        })
        .collect::<Vec<_>>();

    Scroll::new(
        id!(),
        Col::with_gap(10.0, items)
            .padding(10.0)
            .fill(SCHEME.bg1)
            .fit(),
    )
    .sized_wh(300.0, 600.0)
}

fn selectbox() -> UiRef {
    let choices = ["Option 1", "Option 2", "Option 3", "Option 4", "Option 5"];
    let id = id!();

    Col::with_gap(
        20.0,
        [
            flat::SelectBox::new_text(SCHEME.bg1, SCHEME.bg2, id, choices).max_width(300.0),
            text(select_box_value(id).unwrap_or(0)),
        ],
    )
}

fn sized_box() -> UiRef {
    SizedBox::wh(
        300.0,
        200.0,
        BoxFill::new(SCHEME.bg2, Center::new(text("Sized Box"))),
    )
}

fn slider() -> UiRef {
    struct SliderExampleState {
        a: f32,
        b: f32,
        c: f32,
        d: usize,
    }

    storage_init_state::<SliderExampleState>(SliderExampleState {
        a: 10.0,
        b: 5.0,
        c: 15.0,
        d: 5,
    });

    let state = storage_get_state_mut::<SliderExampleState>();

    Col::with_gap(
        20.0,
        [
            Col::with_gap(
                20.0,
                [
                    flat::Slider::new(&mut state.a, 0.0, 30.0, id!()),
                    flat::Slider::alternate(&mut state.b, 0.0, 30.0, id!()),
                    flat::Slider::rounded(&mut state.c, 0.0, 30.0, id!()),
                    fancy_slider(&mut state.d),
                ],
            )
            .max_width(300.0),
            text(
                "Sliders can be quite complicated to use, since they let you decide exactly how it should look. The source code for the flat::Slider is a good place to start. Another example is the fancy_slider function in this example.",
            ),
        ],
    )
}

fn fancy_slider(value: &mut usize) -> UiRef {
    let max = 10;
    let bar = MultipointGradientFill::horizontal(
        (0..10)
            .map(|n| {
                let hue = n as f32 * 36.0;
                let color = Color::from_hsl(hue, 1.0, 0.5);
                GradientPoint::new(color, 1.0)
            })
            .collect::<Vec<_>>(),
        EMPTY,
    )
    .min_height(10.0)
    .padding_vertical(10.0);
    let handle = Fill::builder()
        .color(Color::WHITE)
        .corner_radius(5.0)
        .shadow_color(Color::BLACK)
        .shadow_radius(10.0)
        .child(
            Text::mono_colored(*value, Color::NEUTRAL_500)
                .center_horizontal()
                .padding_top(3.0),
        )
        .build()
        .sized_wh(20.0, 30.0);

    Slider::new(value, 0, max, handle, bar, id!())
}

fn multipoint_gradient_fill() -> UiRef {
    MultipointGradientFill::horizontal(
        (0..10)
            .map(|n| {
                let hue = n as f32 * 36.0;
                let color = Color::from_hsl(hue, 1.0, 0.5);
                GradientPoint::new(color, 1.0)
            })
            .collect::<Vec<_>>(),
        EMPTY,
    )
    .square(200.0)
}

fn stack() -> UiRef {
    let square = BoxFill::new(SCHEME.bg3, EmptyBox::new(50.0, 50.0));

    Col::with_gap(
        20.0,
        [
            BoxFill::new(
                SCHEME.bg1,
                Stack::new([
                    Align::top_left(square),
                    Align::top_center(square),
                    Align::top_right(square),
                    Align::center_right(square),
                    Align::bottom_right(square),
                    Align::bottom_center(square),
                    Align::bottom_left(square),
                    Align::center_left(square),
                    Align::center(square),
                ]),
            )
            .max_size(Vec2::splat(300.0)),
            text("Overlays elements on top of eachother"),
        ],
    )
}

fn window() -> UiRef {
    let size = vec2(400.0, 600.0);
    let id = id!();

    if key_pressed(KeyCode::Escape) {
        let state = floating_window_state(id).unwrap();
        state.open = !state.open;
    }

    Stack::new([
        text("Press Esc to toggle window."),
        flat::FloatingWindow::custom(
            "Example window",
            size,
            vec2((window_width() - size.x) / 2.0, 100.0),
            id,
            Center::new(text("This is a floating window. You can drag it around!")),
        )
        .max_width(300.0),
    ])
}

fn text(text: impl ToString) -> UiRef {
    Text::new_with_color(text, SCHEME.fg0)
}

fn rich_text_page() -> UiRef {
    struct State {
        rich_text: RichText,
    }

    if !storage_exists::<State>() {
        storage_store_state(State {
            rich_text: rich_text(
                r#"<font size=30><font italic color=green5>Uncommon</font> items, like the <font color=purple5><ul>Magic power gem</ul></font> can be very <font bold color=red5>powerful</font></font><font size=50 color=yellow5 bold>!!!</font>"#,
            ).unwrap(),
        });
    }

    let rich_text = storage_get_state::<State>().rich_text.clone();
    let dbg = format!("{:#?}", &rich_text);

    Col::with_gap(40.0, [RichTextNode::new(rich_text), Text::mono_nowrap(dbg)])
        .padding_bottom(100.0)
        .scroll(id!())
}

fn hyperlink() -> UiRef {
    flat::Hyperlink::new("https://google.com")
}

fn dock() -> UiRef {
    fn d(a: UiRef, b: UiRef, orientation: Orientation, split: Split, id: usize) -> UiRef {
        Dock::new(
            BorderStyle::custom(1.0, SCHEME.bg3, BorderType::Rectangle),
            BorderStyle::custom(2.0, SCHEME.bg4, BorderType::Rectangle),
            orientation,
            split,
            id,
            Center::new(a),
            Center::new(b),
        )
    }

    d(
        d(
            d(
                text("A"),
                text("B"),
                Orientation::Vertical,
                Split::Proportion(0.5),
                id!(),
            ),
            d(
                text("C"),
                text("D"),
                Orientation::Vertical,
                Split::FixedSizeB(100.0),
                id!(),
            ),
            Orientation::Horizontal,
            Split::Proportion(0.2),
            id!(),
        ),
        d(
            text("E"),
            d(
                text("F"),
                text("G"),
                Orientation::Horizontal,
                Split::Proportion(0.5),
                id!(),
            ),
            Orientation::Horizontal,
            Split::Proportion(0.8),
            id!(),
        ),
        Orientation::Vertical,
        Split::Proportion(0.5),
        id!(),
    )
    .border(SCHEME.bg3, 1.0)
    .padding_bottom(60.0)
    .max_height(window_height())
}

fn tooltip() -> UiRef {
    Grid::with_gap(
        3,
        3,
        20.0,
        [
            TooltipPosition::Top,
            TooltipPosition::Bottom,
            TooltipPosition::Left,
            TooltipPosition::Right,
            TooltipPosition::AboveCursorRight,
            TooltipPosition::BelowCursorRight,
            TooltipPosition::AboveCursorLeft,
            TooltipPosition::BelowCursorLeft,
        ]
        .into_iter()
        .map(|pos| {
            flat::Tooltip::text(
                pos,
                SizedBox::square(
                    200.0,
                    Fill::new(
                        SCHEME.bg2,
                        Center::new(text(format!("Hover me ({:?})", pos))),
                    ),
                ),
                Text::nowrap("Add anything you want here"),
            )
        })
        .collect::<Vec<_>>(),
    )
    .fit()
}

fn modal() -> UiRef {
    let button_id = id!();
    let delete_button_id = id!();
    let modal_id = id!();

    struct State {
        open: bool,
    }

    if !storage_exists::<State>() {
        storage_store_state(State { open: false });
    }

    let state = storage_get_state_mut::<State>();

    if button_clicked_last_frame(button_id) {
        state.open = !state.open;
    }

    if button_clicked_last_frame(delete_button_id) {
        log::warn!("Account deleted!");
        state.open = false;
    }

    Col::new([
        flat::Modal::new(
            "Are you sure?",
            &mut state.open,
            modal_id,
            Col::with_gap(
                20.0,
                [
                    Text::nowrap("This is a destructive action"),
                    Text::nowrap("Maybe you should think again..."),
                    FlexRow::with_gap(
                        10.0,
                        [
                            FlexBox::Flex(EMPTY),
                            FlexBox::Fixed(flat::Button::danger_text(
                                delete_button_id,
                                "Delete account",
                            )),
                            FlexBox::Fixed(flat::Button::primary_text(button_id, "Cancel")),
                        ],
                    ),
                ],
            ),
        ),
        flat::Button::danger(button_id, Text::nowrap("Delete account")),
    ])
}

fn drop_file() -> UiRef {
    struct State {
        dropped_files: Vec<String>,
    }

    if !storage_exists::<State>() {
        storage_store_state(State {
            dropped_files: vec![],
        });
    }

    let state = storage_get_state_mut::<State>();

    let id = id!();

    if let Some(path) = drop_file_path(id) {
        state.dropped_files.push(path.to_string_lossy().to_string());
    }

    Col::with_gap(
        20.0,
        [
            flat::DropFile::new(vec2(600.0, 400.0), id),
            Col::new(
                state
                    .dropped_files
                    .iter()
                    .map(|s| Text::new(s))
                    .collect::<Vec<_>>(),
            ),
        ],
    )
}

fn search() -> UiRef {
    let options = [
        "Madvillany",
        "Money Store",
        "OK COMPUTER",
        "Exmilitary",
        "The Powers That B",
        "Untrue",
        "Wish You Were Here",
        "Atrocity Exhibition",
        "Bottomless Pit",
        "Enter The Wu-Tang",
        "OFFLINE!",
        "Kid A",
        "To Pimp a Butterfly",
        "Good Kid m.A.A.d City",
        "In Rainbows",
        "Remain in Light",
        "Twin Fantasy ",
        "Chaser",
        "Dark Side of the Moon",
        "Vaudeville Villain",
        "Maxinquaye",
        "Dummy",
        "In the Aeroplane Over the Sea",
        "Vespertine",
    ];

    flat::Search::text(id!(), options)
}

fn flow() -> UiRef {
    struct State {
        boxes: Vec<(u8, f32)>,
    }

    fn b(s: u8, c: f32) -> UiRef {
        let s = s as f32 / 4.0 + 50.0;
        let n = (c * 11.0 + time()) as usize % 11;
        BoxFill::new(Palette::SKY.shades()[n], EMPTY).sized_wh(s, 50.0)
    }

    if !storage_exists::<State>() {
        let boxes = (0..500).map(|_| (rand(), rand())).collect();
        storage_store_state(State { boxes });
    }

    let state = storage_get_state::<State>();
    let children = state
        .boxes
        .iter()
        .map(|(s, c)| b(*s, *c))
        .collect::<Vec<_>>();

    BoxFill::new(
        SCHEME.bg1,
        Flow::horizontal_with_gap(10.0, children)
            .scroll(id!())
            .padding(10.0),
    )
    .fit_size()
}

fn outline() -> UiRef {
    Col::with_gap(
        20.0,
        [
            Row::with_gap(
                20.0,
                [
                    Outline::all_style(
                        BorderStyle::custom(10.0, SCHEME.fg2, BorderType::Dashed(30.0)),
                        BoxFill::new(SCHEME.bg3, Center::new(text("Outline"))),
                    )
                    .square(200.0),
                    Border::all_style(
                        BorderStyle::custom(10.0, SCHEME.fg2, BorderType::Dashed(30.0)),
                        BoxFill::new(SCHEME.bg3, Center::new(text("Border"))),
                    )
                    .square(200.0),
                ],
            ),
            Text::new("Outline does not shift the layout, and is applied over the child"),
        ],
    )
}

fn text_showcase() -> UiRef {
    fn bold_italic_colored(text: impl ToString, color: Color) -> UiRef {
        Text::new_full(text, SANS_BOLD_ITALIC, 24, color, true, 1.0, false)
    }

    Scroll::new(
        id!(),
        Col::new([
            Text::title("Title"),
            Text::body(
                "Lorem ipsum dolor sit amet, consectetur\n\n adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
            ),
            Text::h1("Heading 1"),
            Text::body("Lorem ipsum dolor sit amet."),
            Text::h2("Heading 1"),
            Text::body("Lorem ipsum dolor sit amet."),
            Text::mono_sized(
                format!(
                    "{} {} {} {}",
                    ICON_PLUS_CIRCLE, ICON_BOMB, ICON_CARET_DOWN, ICON_CIRCLE_NOTCH
                ),
                40,
            ),
            Text::h3("Heading 1"),
            Text::body("Lorem ipsum dolor sit amet."),
            Text::italic("Lorem ipsum dolor sit amet."),
            Text::bold("Lorem ipsum dolor sit amet."),
            Text::bold_italic("Lorem ipsum dolor sit amet."),
            Col::new(
                Palette::NEUTRAL
                    .shades()
                    .map(|c| bold_italic_colored("Lorem ipsum dolor sit amet.", c)),
            ),
        ]),
    )
}
