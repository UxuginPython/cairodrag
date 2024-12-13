// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
//!# CairoDrag
//!**An unofficial drag-and-drop implementation for cairo-rs and gtk4.**
#![warn(missing_docs)]
use cairo::{Context, Error};
use glib::Object;
use gtk4::{cairo, glib, prelude::*, subclass::prelude::*, DrawingArea};
use std::cell::RefCell;
use std::rc::Rc;
mod imp;
///An object that is rendered on a Cairo Context and can be dragged.
pub trait Draggable {
    ///Draws the object on a Cairo Context.
    fn draw(&self, context: &Context, x: f64, y: f64) -> Result<(), Error>;
    ///Returns how far the object extends from the coordinates given in `draw` as a tuple of
    ///`(-x, +x, -y, +y)`. These should be positive in all directions, e.g., a centered circle with
    ///a radius of 50 should return `(50.0, 50.0, 50.0, 50.0)`.
    fn get_limits(&self) -> (f64, f64, f64, f64);
    ///Given relative coordinates with the object's last draw at the origin, returns whether the
    ///clicked point should serve as a "handle" for dragging the object. The default implementation
    ///assumes the object is a solid rectangle and uses `get_limits` to calculate if the point is
    ///contained.
    fn contains(&self, x: f64, y: f64) -> bool {
        let (neg_x, pos_x, neg_y, pos_y) = self.get_limits();
        x >= -neg_x && x <= pos_x && y >= -neg_y && y <= pos_y
    }
}
glib::wrapper! {
    ///A subclass of [`gtk4::DrawingArea`] allowing for drag-and-drop of objects implementing the
    ///[`Draggable`] trait.
    pub struct DragArea(ObjectSubclass<imp::DragArea>)
        @extends DrawingArea, gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget;
}
impl DragArea {
    ///Constructor for `DragArea`.
    pub fn new(width: i32, height: i32) -> Self {
        Object::builder()
            .property("width_request", width)
            .property("height_request", height)
            .build()
    }
    pub fn with_boundedness(
        width: i32,
        height: i32,
        boundedness: (bool, bool, bool, bool),
    ) -> Self {
        let output = Self::new(width, height);
        let output_imp = imp::DragArea::from_obj(&output);
        output_imp.set_boundedness(boundedness);
        output
    }
    pub fn new_unbounded(width: i32, height: i32) -> Self {
        Self::with_boundedness(width, height, (false, false, false, false))
    }
    ///Adds a draggable object contained in a `Box` to the `DragArea`.
    pub fn push_box(&self, item: Box<impl Draggable + 'static>, x: f64, y: f64) {
        let self_imp = imp::DragArea::from_obj(self);
        self_imp.push_box(item, x, y);
        self.queue_draw();
    }
    ///Adds a draggable object contained in an `Rc` to the `DragArea`.
    pub fn push_rc(&self, item: Rc<impl Draggable + 'static>, x: f64, y: f64) {
        let self_imp = imp::DragArea::from_obj(self);
        self_imp.push_rc(item, x, y);
        self.queue_draw();
    }
    ///Adds a draggable object contained in an `Rc<RefCell>` to the `DragArea`.
    pub fn push_rc_ref_cell(&self, item: Rc<RefCell<impl Draggable + 'static>>, x: f64, y: f64) {
        let self_imp = imp::DragArea::from_obj(self);
        self_imp.push_rc_ref_cell(item, x, y);
        self.queue_draw();
    }
}
