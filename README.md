# glium_shapes

![Crates.io](https://img.shields.io/crates/v/glium_shapes.svg) ![TravisCI](https://travis-ci.org/jbrd/glium_shapes.svg?branch=master)

Prefabricated shapes for the Glium OpenGL library.

The following shapes are currently provided by the library:

* Axes
* Cuboid
* Quad
* Sphere


## Documentation

Documentation can be found here: [http://jbrd.github.io/glium_shapes](http://jbrd.github.io/glium_shapes)


## Getting Started

- Import the `glium_shapes` crate:

  ```rust
  extern crate glium_shapes;
  ```

- Build a 2x3x4 `Cuboid` with its centre-of-mass at the origin and draw it:

  ```rust
  let cuboid = glium_shapes::cuboid::CuboidBuilder()
               .scale(2.0, 3.0, 4.0)
               .build(display)
               .expect("Failed to build cuboid shape");
  frame.draw(&cuboid, &cuboid, your_shader_program, your_uniforms, your_draw_params);
  ```

- Examples for all shapes are provided. Just run:

  ```bash
  cargo run --example axes
  cargo run --example cuboid
  cargo run --example quad
  cargo run --example sphere
  ```


## Technical Details

* Each shape is constructed using a builder object, which provides methods for customising
  your new shape

* By default, the geometry is constructed to suit the standard OpenGL context defaults:

  * Right-handed coordinate system (x = right, y = up, z = out of screen)
  * Front-faces taken to be counter-clock-wise

* By default, each shape is constructed with its centre-of-mass at the origin


## Development Status

Maintained but not actively developed. Will keep up-to-date with latest versions of rust
and glium. Will add extra shapes when needed, and will happily accept contributions for
extra shapes as well. Bugs will be fixed (please raise an issue if you find any!).


## Contributing Patches

Contributions are most welcome! To reduce the feedback loop, please ensure you perform
the following checks before contributing a patch:

* Ensure the project builds in debug: `cargo build`
* Ensure the project builds in release: `cargo build --release`
* Ensure the tests pass: `cargo test`
* Ensure the coding style is consistent (`cargo fmt` should yield no changes)
* Ensure `cargo clippy --all-targets` returns no warnings or errors
* Ensure `cargo doc` succeeds
* Ensure all the examples build and run successfully
* Ensure your name is added to the contributors list in README.md


## Contributors

James Bird (@jbrd)
