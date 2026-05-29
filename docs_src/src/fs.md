# File System API

SGE provides functions to interact with the file system that integrate with the
engine's `async` system, so that you can read/write to files in the background,
while showing a loading screen for the user. This includes functions for loading
resources like textures, where the image decoding will be done on another thread.

See: [FS module](https://docs.rs/sge/latest/sge/prelude/fs/index.html)

See also: [`/examples/async_asset_loading.rs`](https://github.com/LilyRL/sge/blob/master/examples/async_asset_loading.rs)
