use include_folder::include_folder;
use include_folder::Directory;

include_folder!("/home/josh/repos/chat_axum/build/themes/", "TestDir");

fn main() {
    let dir = test_dir();
    dbg!(dir.glob("*nested*").unwrap());
}
