use crate::Draggable;
use cairo::{Context, Error};
use gtk4::{cairo, glib, prelude::*, subclass::prelude::*, DrawingArea};
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
impl<T: ?Sized> From<Box<T>> for Reference<T> {
    fn from(was: Box<T>) -> Self {
        Self::Box(was)
    }
}
impl<T: ?Sized> From<Rc<T>> for Reference<T> {
    fn from(was: Rc<T>) -> Self {
        Self::Rc(was)
    }
}
impl<T: ?Sized> From<Rc<RefCell<T>>> for Reference<T> {
    fn from(was: Rc<RefCell<T>>) -> Self {
        Self::RcRefCell(was)
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
struct DraggableSetHolder {
    draggables: Vec<Reference<dyn Draggable>>,
    locations: Vec<(f64, f64)>,
}
impl DraggableSetHolder {
    fn new() -> Self {
        Self {
            draggables: Vec::new(),
            locations: Vec::new(),
        }
    }
    fn push(&mut self, item: Reference<dyn Draggable>, x: f64, y: f64) {
        self.draggables.push(item);
        self.locations.push((x, y));
    }
    fn iter(&self) -> DraggableSetHolderIterator<'_> {
        let len = self.draggables.len();
        let index_back = if len >= 1 { len - 1 } else { 0 };
        DraggableSetHolderIterator {
            holder: self, //This is a reference
            index_start: 0,
            index_back: index_back,
        }
    }
}
struct DraggableAndCoordinates<'a> {
    draggable: ReferenceBorrow<'a, dyn Draggable>,
    x: f64,
    y: f64,
}
impl<'a> DraggableAndCoordinates<'a> {
    fn draw(&self, context: &Context) -> Result<(), Error> {
        self.draggable.draw(context, self.x, self.y)
    }
}
struct DraggableSetHolderIterator<'a> {
    holder: &'a DraggableSetHolder,
    index_start: usize,
    index_back: usize,
}
impl<'a> Iterator for DraggableSetHolderIterator<'a> {
    type Item = DraggableAndCoordinates<'a>;
    fn next(&mut self) -> Option<DraggableAndCoordinates<'a>> {
        if self.index_start >= self.holder.draggables.len() || self.index_start > self.index_back {
            return None;
        }
        let output = DraggableAndCoordinates {
            draggable: self.holder.draggables[self.index_start].borrow(),
            x: self.holder.locations[self.index_start].0,
            y: self.holder.locations[self.index_start].1,
        };
        self.index_start += 1;
        Some(output)
    }
}
impl<'a> DoubleEndedIterator for DraggableSetHolderIterator<'a> {
    fn next_back(&mut self) -> Option<DraggableAndCoordinates<'a>> {
        //usize type keeps it from going below zero
        if self.index_back < self.index_start {
            return None;
        }
        let output = DraggableAndCoordinates {
            draggable: self.holder.draggables[self.index_back].borrow(),
            x: self.holder.locations[self.index_back].0,
            y: self.holder.locations[self.index_back].1,
        };
        self.index_back -= 1;
        Some(output)
    }
}

pub struct DragArea {
    draggables: Rc<RefCell<DraggableSetHolder>>,
}
impl DragArea {
    pub fn new() -> Self {
        let draggables = Rc::new(RefCell::new(DraggableSetHolder::new()));
        Self {
            draggables: draggables,
        }
    }
    pub fn push_box(&self, item: Box<impl Draggable + 'static>, x: f64, y: f64) {
        self.draggables
            .borrow_mut()
            .push((item as Box<dyn Draggable>).into(), x, y);
    }
    pub fn push_rc(&self, item: Rc<impl Draggable + 'static>, x: f64, y: f64) {
        self.draggables
            .borrow_mut()
            .push((item as Rc<dyn Draggable>).into(), x, y);
    }
    pub fn push_rc_ref_cell(&self, item: Rc<RefCell<impl Draggable + 'static>>, x: f64, y: f64) {
        self.draggables
            .borrow_mut()
            .push((item as Rc<RefCell<dyn Draggable>>).into(), x, y);
    }
}
impl Default for DragArea {
    fn default() -> Self {
        Self {
            draggables: Rc::new(RefCell::new(DraggableSetHolder::new())),
        }
    }
}
#[glib::object_subclass]
impl ObjectSubclass for DragArea {
    const NAME: &'static str = "CairoDragDragArea";
    type Type = super::DragArea;
    type ParentType = DrawingArea;
}
impl ObjectImpl for DragArea {
    fn constructed(&self) {
        self.parent_constructed();
        let my_draggables = self.draggables.clone();
        self.obj()
            .set_draw_func(move |_drawing_area, context, _width, _height| {
                //for i in my_draggables.borrow().iter().rev() {
                for i in my_draggables.borrow().iter() {
                    i.draw(&context).unwrap();
                }
                //todo!();
            });
    }
}
impl WidgetImpl for DragArea {}
impl DrawingAreaImpl for DragArea {}
