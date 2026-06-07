use sge::sge_include_assets;

// can parse JSON, TOML, and Ron
sge_include_assets!("assets/data", ASSETS);

fn main() {
    println!("{}", ASSETS.test.messages.hello);
    println!("{:?}", ASSETS.users.data.charlie);
    println!("{:#?}", ASSETS.state.window);
}
