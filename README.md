# Freehand

Freehand is a small Rust library with minimal dependencies that provides extra drawing utilities that utilize the [`image`](https://docs.rs/image/latest/image/) crate.

## Why?

- Only depends on the [`image`](https://docs.rs/image/latest/image/) crate.
- Provides some additional drawing functions not found in the [`imageproc`](https://docs.rs/imageproc/latest/imageproc/) (which also has many dependencies).

## Drawing

It provides the following for drawing:
- circular arcs
- partial annulii (basically a slice of a filled donut)
- antialiased circular arcs
- straight vertical, horizontal, and diagonal lines, with variants for dashed lines and alpha blended lines
- filled rectangles

## Usage

The [`image`](https://docs.rs/image/latest/image/) crate should already be a dependency, as this crate operates on its image types.

1. Add freehand to your `cargo.toml`'s dependencies:

    ```toml
    [dependencies]
    freehand = "0.1.0"
    ```

2. Choose to either use the `Draw` struct, for convenience methods, or the regular funcitons (the `Draw` struct methods are recommended).

    ```rust
    // no `use` statements required here
    // just pass in the image you wish to work on
    let draw = freehand::new(&mut image);
    
    let red = image::Rgba([255, 0, 0, 255]);
    
    // draws a line diagonally across an a 400x400 pixel image in red
    draw.line((0, 0), (399,399), red);
    ```



## Limitations

Currently antialiasing and alpha blending are only supported when using an [`RgbaImage`](https://docs.rs/image/latest/image/type.RgbaImage.html) image.

## Todo

- Add more shapes
- Add elliptical arcs


## License

`SPDX-License-Identifier: Apache-2.0 OR MIT`
