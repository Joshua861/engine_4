# Simple Game Engine

SGE requires that you have the Rust toolchain installed, and some familiarity
with rust.

To create a new Rust project, you can run `cargo new my_project`, and replace
my_project with whatever you want to call your game/app. To add SGE to your project run `cargo add sge`.

```sh
cargo new my_project
cd my_project
cargo add sge
```

Throughout this guide there will be links to examples that you can read and use
to find out more about how SGE works and how to use it. To run these, clone the
repository and use `cargo run --example` like so:

```sh
git clone https://github.com/lilyRL/sge
cd sge
cargo run --example name_of_example
```

Do not include the `.rs` in the name of the example.

## Nix

If you use Nix/NixOS, there is a [`shell.nix`](https://github.com/LilyRL/sge/blob/master/shell.nix) included in the repository that you
can use to build and use the engine, though it was written with Wayland in mind.
If you don't know what Nix is you can safely ignore this entirely.
