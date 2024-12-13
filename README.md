# CairoDrag
**An unofficial drag-and-drop implementation for `cairo-rs` and `gtk4`.**

Drag-and-drop interfaces are useful in many places, and [Rust's GTK 4 bindings](https://crates.io/crates/gtk4) have some support for them. However, there are some cases in which GTK's drag-and-drop is not sufficient, requiring the use of [Cairo](https://crates.io/crates/cairo-rs), its drawing library. Unfortunately, Cairo does not have drag-and-drop support by default. This crate adds this functionality.
Read the [documentation](https://docs.rs/cairodrag) and the examples (found in the [repository](https://github.com/UxuginPython/cairodrag)) to get started.

## License: BSD 3-Clause
This basically means that you can do whatever you want as long as you give me attribution and you don't remove the license notices or use my name to endorse stuff I don't. Read the actual license for details though.

## Changes
### 0.1.0
- Initial release.
### 0.1.1
- Change `DragArea::push_(box|rc|rc_ref_cell)` to `&self` instead of `&mut self` since they only rely on interior mutability internally.
