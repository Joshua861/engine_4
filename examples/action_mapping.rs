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
            bind(FWD, KeyCode::KeyE);
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
//     bind(FWD, KeyCode::KeyW);
//     bind(BACK, KeyCode::KeyS);
//     bind(RIGHT, KeyCode::KeyD);
//     bind(LEFT, KeyCode::KeyA);
//     bind(JUMP, KeyCode::Space);
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
