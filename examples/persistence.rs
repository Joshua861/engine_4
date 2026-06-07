use sge::prelude::*;

const PATH: &'static str = "/tmp/config.bin";

#[derive(PartialEq, Debug)]
#[persistent]
struct Config {
    counter: u64,
    foo: Foo,
    color: Color,
}

#[derive(PartialEq, Debug)]
#[persistent]
struct Foo {
    bar: bool,
    baz: Vec2,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            counter: 1,
            foo: Foo {
                bar: false,
                baz: vec2(100.0, 250.0),
            },
            color: Color::RED_300,
        }
    }
}

#[main("Persistence")]
async fn main() -> anyhow::Result<()> {
    let mut config = Config::load(PATH).unwrap_or_else(|e| {
        println!("Could not load config: {e}");
        let _ = std::fs::remove_file(PATH);
        Default::default()
    });

    loop {
        clear_screen(config.color);
        draw_text_ex(config.counter, Vec2::splat(20.0), Color::WHITE, 300);
        sharpen_screen(1.0);

        if key_held(KeyCode::Space) {
            config.counter += 1;
        }

        if key_held(KeyCode::KeyC) {
            config.color = config.color.hue_rotate_oklch(1.0);
        }

        if key_pressed(KeyCode::KeyT) {
            test();
        }

        if key_pressed(KeyCode::KeyD) {
            config = Config::default();
            let _ = std::fs::remove_file(PATH);
        }

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    // please note that if the app is force quit (e.g.: Ctrl-C) this code will not run
    // you might want to do this periodically while the game is running
    config.save(PATH)?;

    Ok(())
}

fn test() {
    let path = "/tmp/config-test.bin";
    let value = Config {
        counter: 19823793,
        foo: Config::default().foo,
        color: Color::BLUE_400,
    };
    value.save(path).unwrap();

    for _ in 0..1000 {
        let loaded = Config::load(path).unwrap();
        loaded.save(path).unwrap();
    }

    let loaded = Config::load(path).unwrap();
    assert_eq!(value, loaded);
    println!("Test suceeded");
}
