# Freehand

Freehand is a small Rust library that provides some extra drawing utilities for the [`image`](https://github.com/image-rs/image) crate.

It provides the following features for drawing:
- circular arcs
- partial annulii - basically a part of a filled donut
- antialiased circular arcs
- straight vertical and horizontal lines, with variants for dashed lines


## Performance

The `blend_at_unchecked()` function for `Rgba<u8>` provides a significant performance improvement over the default [blend()](https://docs.rs/image/latest/image/struct.Rgba.html#method.blend) method - up to 200-300%.


## Unsafe

Provides unsafe functions to remove redundant bounds checks and eliminate unnecessary branches.  Useful when you have already checked the bounds or know the coordinates cannot be outside the bounds.


## Limitations

Currently antialiasing is only supported when using an [`RgbaImage`](https://docs.rs/image/latest/image/type.RgbaImage.html) image.



