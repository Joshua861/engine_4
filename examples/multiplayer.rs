use sge::prelude::*;
use ui::*;

#[derive(Clone, PartialEq)]
#[persistent(diff, lerp)]
pub struct State {
    position: Vec2,
}

const NAMES: &[&str] = &["Alice", "Bob", "Charlie", "Derek", "Emily", "Fred", "Gurt"];
const UPDATE_RATE: f32 = 0.05;
const INTERPOLATION_DELAY: f32 = UPDATE_RATE * 4.0;

#[main("Multiplayer")]
fn main() {
    set_min_log_level(LevelFilter::Debug);
    set_max_drawn_log_lines(50);

    let mut state = MultiplayerState::new(
        State {
            position: Vec2::ZERO,
        },
        rand_choice(NAMES).to_string(),
        "example_room".to_string(),
    );

    state.announce_self().unwrap();

    loop {
        clear_screen(Color::NEUTRAL_900);
        let now = time();

        if once_per_n_seconds(UPDATE_RATE) {
            state.update().unwrap();
        }

        let movement =
            pressed_movement_vector(KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD);
        state.your_state_mut().position += movement * delta_time() * 300.0;

        draw_user(
            state.your_username().to_string(),
            state.your_state().position,
        );

        let render_target_time = now - INTERPOLATION_DELAY;
        for (_, user) in state.other_users().iter() {
            if let Some(interpolated_state) = user.current_lerped(render_target_time) {
                draw_user(user.username.clone(), interpolated_state.position);
            }
        }

        chat(&mut state);

        draw_logs();

        if should_quit() {
            break;
        }
        next_frame().await;
    }

    state.disconnect();
    // so it has time to send the disconnect message before being killed
    std::thread::sleep(Duration::from_millis(50));
}

struct ChatState {
    messages: Vec<RichText>,
}

fn chat(state: &mut MultiplayerState<State>) {
    if !storage_exists::<ChatState>() {
        storage_store_state(ChatState { messages: vec![] });
    }

    let messages = &mut storage_get_state_mut::<ChatState>().messages;

    let other_state = unsafe { &*{ state as *const MultiplayerState<State> } };
    for notification in state.drain_notifications() {
        let Some(user) = other_state.get_user(notification.user_id) else {
            break;
        };
        let username = &user.username;
        let text = String::from_utf8(notification.data).unwrap();

        messages.push(rich_text(format!("<b>{username}</b>: {text}")).unwrap());
    }

    let input_id = id!();
    let button_id = id!();

    if button_clicked_last_frame(button_id) {
        let input_state = text_input_state(input_id);
        messages.push(rich_text(format!("<b>You</b>: {}", input_state.value)).unwrap());
        state.send_notification(input_state.value.as_bytes().to_vec());
        input_state.value = "".to_string();
    }

    let ui = Align::bottom_left(
        BoxFill::new(
            flat::BG0,
            Padding::all(
                20.0,
                Col::new([
                    FlexRow::with_gap(
                        10.0,
                        [
                            FlexBox::Flex(flat::TextInput::new(flat::BG1, input_id)),
                            FlexBox::Fixed(flat::Button::text(
                                flat::BG1,
                                flat::BG2,
                                button_id,
                                "Send",
                            )),
                        ],
                    ),
                    Scroll::new(
                        id!(),
                        Col::with_gap(
                            10.0,
                            messages
                                .iter()
                                .map(|r| RichTextNode::new(r.clone()))
                                .collect::<Vec<_>>(),
                        ),
                    )
                    .padding_vertical(20.0),
                ]),
            ),
        )
        .sized_wh(600.0, 400.0),
    )
    .sized(window_size());

    draw_ui(ui, Vec2::ZERO);
}

fn draw_user(username: String, pos: Vec2) {
    draw_circle_world(pos, 50.0, Color::BLUE_500);
    draw_text_world(&username, pos);
}
