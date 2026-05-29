# Gamepad

You can get a handle to the gamepad input state with [`gamepad::input()`](https://docs.rs/sge/latest/sge/prelude/gamepad/fn.input.html). With
this, you can get an iterator of the connected gamepads and their [IDs](https://docs.rs/sge/latest/sge/prelude/gamepad/struct.GamepadId.html) with [`gamepad::input().gamepads()`](https://docs.rs/sge/latest/sge/prelude/gamepad/struct.Gilrs.html#method.gamepads).

Using a [`Gamepad`](https://docs.rs/sge/latest/sge/prelude/gamepad/struct.Gamepad.html), which can also be obtained from
[`gamepad::input().gamepad(id)`](https://docs.rs/sge/latest/sge/prelude/gamepad/struct.Gilrs.html#method.gamepad), you can query the state of the buttons and
sticks using the methods on the [`Gamepad`](https://docs.rs/sge/latest/sge/prelude/gamepad/struct.Gamepad.html) struct.

Importantly:
- `.right_stick`, `.left_stick`, and `.d_pad` for getting the input of
  sticks/dpad as a vector.
- `.is_pressed` for checking if a
  [`gamepad::Button`](https://docs.rs/sge/latest/sge/prelude/gamepad/enum.Button.html)
  is pressed
  
---
   
See: [gamepad module](https://docs.rs/sge/latest/sge/prelude/gamepad/index.html)

See also:
[`/examples/gamepad.rs`](https://github.com/LilyRL/sge/blob/master/examples/gamepad.rs)
for an example on how to use the API, and to test if your controller is being
recognised properly.
