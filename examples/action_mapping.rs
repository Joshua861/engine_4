use engine_4::prelude::*;

actions! {
    FWD, BACK, RIGHT, LEFT, JUMP, REBIND
}

fn main() -> anyhow::Result<()> {
    init("Action mapping")?;

    bind! {
        FWD => KeyCode::KeyW;
        BACK => KeyCode::KeyS;
        RIGHT => KeyCode::KeyD;
        LEFT => KeyCode::KeyA;
        JUMP => KeyCode::Space;
        REBIND => KeyCode::KeyR;
    }

    loop {
        if action_pressed(FWD) {
            println!("FWD pressed");
        }

        if action_pressed(REBIND) {
            bind_key(FWD, KeyCode::KeyE);
        }

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}

// // --- Generated Code (slightly simplified) ---
//
// use engine_4::prelude::*;
//
// const FWD: Action = Action::new(0);
// const BACK: Action = Action::new(1);
// const RIGHT: Action = Action::new(2);
// const LEFT: Action = Action::new(3);
// const JUMP: Action = Action::new(4);
//
// fn main() -> anyhow::Result<()> {
//     init("Action mapping")?;
//
//     bind_button(FWD, KeyCode::KeyW.into());
//     bind_button(BACK, KeyCode::KeyS.into());
//     bind_button(RIGHT, KeyCode::KeyD.into());
//     bind_button(LEFT, KeyCode::KeyA.into());
//     bind_button(JUMP, KeyCode::Space.into());
//
//     loop {
//         if action_pressed(FWD) {
//             println!("FWD")
//         }
//
//         if should_quit() {
//             break;
//         }
//
//         next_frame();
//     }
//     Ok(())
// }
