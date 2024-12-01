use cairo::{Context, Error};
use gtk4::{cairo, glib::clone, prelude::*, DrawingArea};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
enum Reference<T: ?Sized> {
    Box(Box<T>),
    Rc(Rc<T>),
    RcRefCell(Rc<RefCell<T>>),
}
impl<T: ?Sized> Reference<T> {
    fn borrow(&self) -> ReferenceBorrow<'_, T> {
        match self {
            Self::Box(boxx) => ReferenceBorrow::NormalReference(boxx),
            Self::Rc(rc) => ReferenceBorrow::NormalReference(rc),
            Self::RcRefCell(rc_ref_cell) => ReferenceBorrow::RefCellBorrow(rc_ref_cell.borrow()),
        }
    }
}
enum ReferenceBorrow<'a, T: ?Sized> {
    NormalReference(&'a T),
    RefCellBorrow(std::cell::Ref<'a, T>),
}
impl<T: ?Sized> Deref for ReferenceBorrow<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        match self {
            Self::NormalReference(x) => x,
            Self::RefCellBorrow(x) => x,
        }
    }
}
pub trait Draggable {
    fn draw(&self, context: &Context, x: f64, y: f64) -> Result<(), Error>;
}
struct DraggableSetHolder {
    draggables: Vec<Reference<dyn Draggable>>,
}
impl DraggableSetHolder {
    fn new() -> Self {
        Self {
            draggables: Vec::new(),
        }
    }
    fn iter(&self) -> DraggableSetHolderIterator<'_> {
        DraggableSetHolderIterator {
            holder: self, //This is a reference
            index: 0,
        }
    }
}
struct DraggableSetHolderIterator<'a> {
    holder: &'a DraggableSetHolder,
    index: usize,
}
impl<'a> Iterator for DraggableSetHolderIterator<'a> {
    type Item = ReferenceBorrow<'a, dyn Draggable>;
    fn next(&mut self) -> Option<ReferenceBorrow<'a, dyn Draggable>> {
        if self.index >= self.holder.draggables.len() {
            return None;
        }
        let output = self.holder.draggables[self.index].borrow();
        self.index += 1;
        Some(output)
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
                /*draggables.borrow().for_each(|x| {
                    x.draw(&context, 0.0, 0.0).unwrap();
                });*/
                todo!();
            }
        ));
        Self {
            drawing_area: drawing_area,
            draggables: draggables,
        }
    }
}
