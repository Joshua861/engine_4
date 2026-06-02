use sge::prelude::*;

fn main() {
    init_custom(
        Opts::builder()
            .log_verbosity(Verbosity::High)
            .title(String::new())
            .build(),
    )
    .unwrap();
    println!("{:?}", opengl_version());
}
