# Post-processing effects

You can apply post processsing effects onto the screen with one of the following functions:

- [`bloom_screen`](https://docs.rs/sge/latest/sge/prelude/post_processing/fn.bloom_screen.html)
- [`blur_screen`](https://docs.rs/sge/latest/sge/prelude/post_processing/fn.blur_screen.html)
- [`brighten_screen`](https://docs.rs/sge/latest/sge/prelude/post_processing/fn.brighten_screen.html)
- [`chromatic_abberation_screen`](https://docs.rs/sge/latest/sge/prelude/post_processing/fn.chromatic_abberation_screen.html)
- [`contrast_screen`](https://docs.rs/sge/latest/sge/prelude/post_processing/fn.contrast_screen.html)
- [`film_grain_screen`](https://docs.rs/sge/latest/sge/prelude/post_processing/fn.film_grain_screen.html)
- [`greyscale_screen`](https://docs.rs/sge/latest/sge/prelude/post_processing/fn.greyscale_screen.html)
- [`hue_rotate_screen`](https://docs.rs/sge/latest/sge/prelude/post_processing/fn.hue_rotate_screen.html)
- [`invert_screen`](https://docs.rs/sge/latest/sge/prelude/post_processing/fn.invert_screen.html)
- [`pixelate_screen`](https://docs.rs/sge/latest/sge/prelude/post_processing/fn.pixelate_screen.html)
- [`saturate_screen`](https://docs.rs/sge/latest/sge/prelude/post_processing/fn.saturate_screen.html)
- [`sharpen_screen`](https://docs.rs/sge/latest/sge/prelude/post_processing/fn.sharpen_screen.html)
- [`vignette_screen`](https://docs.rs/sge/latest/sge/prelude/post_processing/fn.vignette_screen.html)

These effects will be applied to everything that has already been rendered out
on the screen, but not anything that was rendered afterwards. This will also
create new draw queues.

---

See: [post processing module](https://docs.rs/sge/latest/sge/prelude/post_processing/index.html)
