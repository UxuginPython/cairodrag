use gtk4::prelude::*;
use gtk4::{cairo, glib, DrawingArea};
use cairo::{Context, Error};
use glib::clone;
use std::cell::RefCell;
use std::rc::Rc;
pub trait Draggable {
    fn draw(&self, context: &Context, x: f64, y: f64) -> Result<(), Error>;
}
pub struct DragArea {
    drawing_area: Rc<DrawingArea>,
    draggables: Rc<RefCell<Vec<Box<dyn Draggable>>>>,
}
impl DragArea {
    pub fn new(width: i32, height: i32) -> Self {
        let draggables = Rc::new(RefCell::new(Vec::<Box<dyn Draggable>>::new()));
        let drawing_area = Rc::new(DrawingArea::builder().content_width(width).content_height(height).build());
        drawing_area.set_draw_func(clone!(@strong drawing_area, @strong draggables => move |_drawing_area: &DrawingArea, context: &Context, _width: i32, _height: i32| {
            for i in draggables.borrow().iter().rev() {
                i.draw(&context, 0.0, 0.0).unwrap();
            }
        }));
        Self {
            drawing_area: drawing_area,
            draggables: draggables,
        }
    }
}
