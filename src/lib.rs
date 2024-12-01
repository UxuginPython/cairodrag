use cairo::{Context, Error};
use gtk4::{cairo, glib::clone, prelude::*, DrawingArea};
use std::cell::RefCell;
use std::rc::Rc;
pub trait Draggable {
    fn draw(&self, context: &Context, x: f64, y: f64) -> Result<(), Error>;
}
struct DraggableSetHolder {
    draggables: Vec<Rc<RefCell<dyn Draggable>>>,
}
impl DraggableSetHolder {
    fn new() -> Self {
        Self {
            draggables: Vec::new(),
        }
    }
    /*fn iter(&self) -> std::slice::Iter<Rc<RefCell<dyn Draggable>>> {
        self.draggables.iter()
    }*/
    fn for_each(&self, mut func: impl FnMut(&dyn Draggable) -> ()) {
        for i in &self.draggables {
            func(&*i.borrow());
        }
    }
}
pub struct DragArea {
    drawing_area: DrawingArea,
    draggables: Rc<RefCell<DraggableSetHolder>>,
}
impl DragArea {
    pub fn new(width: i32, height: i32) -> Self {
        let draggables = Rc::new(RefCell::new(DraggableSetHolder::new()));
        let drawing_area = DrawingArea::builder()
            .content_width(width)
            .content_height(height)
            .build();
        drawing_area.set_draw_func(clone!(
            #[strong]
            draggables,
            move |_drawing_area, context, _width, _height| {
                /*for i in draggables.borrow().iter().rev() {
                    i.borrow().draw(&context, 0.0, 0.0).unwrap();
                }*/
                draggables.borrow().for_each(|x| {
                    x.draw(&context, 0.0, 0.0).unwrap();
                });
            }
        ));
        Self {
            drawing_area: drawing_area,
            draggables: draggables,
        }
    }
}
