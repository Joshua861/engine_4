#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use include_folder::include_folder;
use include_folder::Directory;
fn main() {
    let dir = test_dir();
    match dir.glob("*nested*").unwrap() {
        tmp => {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3} = {4:#?}\n",
                        "src/main.rs",
                        8u32,
                        5u32,
                        "dir.glob(\"*nested*\").unwrap()",
                        &tmp,
                    ),
                );
            };
            tmp
        }
    };
}
