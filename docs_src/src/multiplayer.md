# Multiplayer

Multiplayer netcode for video games can be really hard to write. SGE has a
system to make this task easier.

First, create a state struct to hold the data you want associated with each
player. This struct must implement `Clone`, and be annotated with
`#[persistent(diff)]`. The diff is to allow the multiplayer system to tell what
part of the struct changed between updates, so it can efficiently only send
what's necessary.

```rust
#[derive(Clone)]
#[persistent(diff)]
struct State {
    position: Vec2,
}
```

Then create a `MultiplayerState<T>` object with the default instance of the
struct, a username, and room name. Your player will be connected to all other
players in the same room, so make sure that it is, at least, unique to the video
game you are developing by adding some random characters at the top for all
rooms part of your game.

```rust
let mut state = MultiplayerState::new(
    State {
        position: Vec2::ZERO,
    },
    "Lily".to_string(),
    "YOUR_GAME_NAME_1897".to_string(),
);
```

From this point, you can call `state.update()`, at a rate of your choosing. I
would recommend not sending it too often as this is bad for performance and will
use more bandwidth, I would recommend 10 or 20 times a second.

```rust
// 10 times a second
const UPDATE_RATE: f32 = 0.1;

fn main() -> anyhow::Result<()> {
    // ...
    
    loop {
        // ...
        
        if once_per_n_seconds(UPDATE_RATE) {
            state.update()?;
        }
    }
}
```

You can access your state (i.e. the state of the player) with
`state.your_state()`, `state.your_state_mut()`, `state.your_username()`, and
`state.your_user_data()`. You can get the states of other users with
`state.other_users()`, `state.get_user()`, and `state.get_user_mut()`. Note that
any changes you make to other users states will not be reflected for other
people, and will be updated by new data from that user changing their own state.

In your cleanup (end of main function), it is best to add this code:

```rust
state.disconnect();
// so it has time to send the disconnect message before the process is killed
std::thread::sleep(Duration::from_millis(50));
```

This will tell all other clients in the room that you have disconnected, and
will remove that user from their list of users, so they won't still show up as a
frozen player in game.

## Interpolation

If you create something like this:

```rust
struct State {
    position: Vec2,
}

const UPDATE_RATE: f32 = 0.1;

#[main("Multiplayer")]
fn main() {
    let mut state = MultiplayerState::new(
        State {
            position: Vec2::ZERO,
        },
        "Lily".to_string(),
        "YOUR_GAME_NAME_1897".to_string(),
    );
    
    loop {
        // add controls for user to move around, and update the state periodically
        
        for (_, user) in state.other_users().iter() {
            draw_circle_world(user.position, 50.0, Color::RED_500);
        }
        
        // ...
    }
}
```

...you will notice that people jump around on screen in large intervals, because
their positions are being updated at a rate less than the frame rate. To fix
this, without reducing performance, we can use interpolation. You can implement
interpolation manually using the history of previous state values stored in
`UserData`, or use the builtin automatic interpolation. To use automatic
interpolation, you need to annotate the state struct with `#[persistent(diff,
lerp)]` instead.

```rust
#[derive(Clone)]
#[persistent(diff, lerp)]
struct State {
    position: Vec2,
}
```

This will implement `PartialLerp` for that type, which interpolates only the
fields that can be interpolated (`f32`, `f64`, `Vec2`, `Vec3`, `Vec4`, `Color`).
With this, on types that implement `PartialLerp`, you can use something like
this instead, for smooth interpolation without suttering even at low update rates.

```rust
const UPDATE_RATE: f32 = 0.1;
const INTERPOLATION_DELAY: f32 = UPDATE_RATE * 2.0; // can add more for more reliability; 2*update-rate is standard

// ...

// render target time is the time in the past we should pretend it is,
// and interpolate based on updates we have gotten from the future (aka present)
let render_target_time = time() - INTERPOLATION_DELAY;
for (_, user) in state.other_users().iter() {
    if let Some(interpolated_state) = user.current_lerped(render_target_time) {
        draw_circle_world(user.position, 50.0, Color::RED_500);
    }
}
```

## Notifications

If you want to communicate directly with other clients, for example when adding
a chat system to your game, you can use notifications. Send a notification with
`state.send_notification(data)`, and recieve with `state.drain_notifications()`.
The data sent in a notification is a `Vec<u8>`,
allowing you to send any data you want in any form. You could do this by
reserving the first number in the series as a type specifier, and interpreting
the rest of the sequence based on the parsed type. Remember that you can convert
any type annotated with `#[persistent]` to bytes with `.to_bytes()`, and convert
strings to and from bytes with `string.as_bytes()` and `String::from_utf8()`.

```rust
let mut state = MultiplayerState::new(...);

let mut messages = vec![];

for notification in state.drain_notifications() {
    let Some(user) = other_state.get_user(notification.user_id) else {
        break;
    };
    let username = &user.username;
    let text = String::from_utf8(notification.data).unwrap();

    messages.push(text);
}

let message = "hello".to_string();
state.send_notification(message.as_bytes().to_vec());
```

See: [multiplayer module](https://docs.rs/sge/latest/sge/prelude/multiplayer/index.html)

See: [`/examples/multiplayer.rs`](https://github.com/LilyRL/sge/blob/master/examples/multiplayer.rs)
