# Image API

[`Image`](https://docs.rs/sge/latest/sge/prelude/image/struct.Image.html)s are
like textures, but stored on the CPU. Images can be loaded and parsed
from image files like `.png` and `.jpg`, and their contents can be manipulated
by indexing into the buffer directly or using the many methods on the
[`Image`](https://docs.rs/sge/latest/sge/prelude/image/struct.Image.html) struct
to draw shapes.

Unlike when GPU rendering, working with an `Image` requires you to use the
[`ColorU8`](https://docs.rs/sge_color/latest/sge_color/u8/union.ColorU8.html)
type, which uses u8 values for (r,g,b,a), that range from 0 to 255,
where white is (255,255,255,255). `ColorU8` can be converted to `Color` and vice
versa with `.to_color` and `.to_color_u8`.

An image can be uploaded to a GPU texture with
[`SgeTexture::from_enginen_image`](https://docs.rs/sge/latest/sge/prelude/textures/struct.SgeTexture.html#method.from_engine_image)
and a GPU texture can be downloaded to an image with `texture.download_to_image()`.

---
   
See: [image module](https://docs.rs/sge/latest/sge/prelude/image/index.html)
