use cairo::{Context, Error};
use glib::Object;
use gtk4::{cairo, glib, subclass::prelude::*, DrawingArea};
mod imp;
pub trait Draggable {
    fn draw(&self, context: &Context, x: f64, y: f64) -> Result<(), Error>;
    ///(-x, +x, -y, +y)
    fn get_limits(&self) -> (f64, f64, f64, f64);
    ///RELATIVE
    fn contains(&self, x: f64, y: f64) -> bool {
        let (neg_x, pos_x, neg_y, pos_y) = self.get_limits();
        x >= -(neg_x.abs()) && x <= pos_x && y >= -(neg_y.abs()) && y <= pos_y
    }
}
glib::wrapper! {
    pub struct DragArea(ObjectSubclass<imp::DragArea>)
        @extends DrawingArea, gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget;
}
impl DragArea {
    pub fn new(width: i32, height: i32) -> Self {
        Object::builder()
            .property("width_request", width)
            .property("height_request", height)
            .build()
    }
    pub fn push_box(&mut self, item: Box<impl Draggable + 'static>, x: f64, y: f64) {
        let selff = imp::DragArea::from_obj(self);
        selff.push_box(item, x, y);
    }
}
