// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
use crate::Draggable;
use cairo::{Context, Error};
use gtk4::{cairo, glib, prelude::*, subclass::prelude::*, DrawingArea, GestureDrag};
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
    fn move_to_end(&mut self, index: usize) -> usize {
        let element = self.draggables.remove(index);
        self.draggables.push(element);
        let element = self.locations.remove(index);
        self.locations.push(element);
        self.draggables.len() - 1
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
struct DragInfo {
    start_x: f64,
    start_y: f64,
    index: usize,
    relative_x: f64,
    relative_y: f64,
}

pub struct DragArea {
    draggables: Rc<RefCell<DraggableSetHolder>>,
    drag_info: Rc<RefCell<Option<DragInfo>>>,
}
impl DragArea {
    pub fn new() -> Self {
        let draggables = Rc::new(RefCell::new(DraggableSetHolder::new()));
        Self {
            draggables: draggables,
            drag_info: Rc::new(RefCell::new(None)),
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
            drag_info: Rc::new(RefCell::new(None)),
        }
    }
}
#[glib::object_subclass]
impl ObjectSubclass for DragArea {
    const NAME: &'static str = "CairoDragDragArea";
    type Type = super::DragArea;
    type ParentType = DrawingArea;
}
//                                                  width or height, whichever we're calculating
fn calculate_limits(neg_limit: f64, pos_limit: f64, area_size: i32, desired_coord: f64) -> f64 {
    if desired_coord < neg_limit {
        return neg_limit;
    }
    if desired_coord > area_size as f64 - pos_limit {
        return area_size as f64 - pos_limit;
    }
    desired_coord
}
impl ObjectImpl for DragArea {
    fn constructed(&self) {
        self.parent_constructed();
        let my_draggables = self.draggables.clone();
        self.obj()
            .set_draw_func(move |_drawing_area, context, _width, _height| {
                for i in my_draggables.borrow().iter() {
                    i.draw(&context).unwrap();
                }
            });
        let drag = GestureDrag::new();
        let my_draggables = self.draggables.clone();
        let my_drag_info = self.drag_info.clone();
        let my_obj = self.obj().clone();
        drag.connect_drag_begin(move |_gesture: &GestureDrag, x: f64, y: f64| {
            let mut new_drag_info = None;
            for (i, draggable_and_coords) in my_draggables.borrow().iter().enumerate() {
                if draggable_and_coords
                    .draggable
                    .contains(x - draggable_and_coords.x, y - draggable_and_coords.y)
                {
                    new_drag_info = Some(DragInfo {
                        start_x: x,
                        start_y: y,
                        index: i,
                        relative_x: draggable_and_coords.x - x,
                        relative_y: draggable_and_coords.y - y,
                    })
                }
            }
            new_drag_info = match new_drag_info {
                Some(drag_info) => Some(DragInfo {
                    start_x: drag_info.start_x,
                    start_y: drag_info.start_y,
                    index: my_draggables.borrow_mut().move_to_end(drag_info.index),
                    relative_x: drag_info.relative_x,
                    relative_y: drag_info.relative_y,
                }),
                None => None,
            };
            *my_drag_info.borrow_mut() = new_drag_info;
            my_obj.queue_draw();
        });
        let my_draggables = self.draggables.clone();
        let my_drag_info = self.drag_info.clone();
        let my_obj = self.obj().clone();
        drag.connect_drag_update(move |_gesture: &GestureDrag, x: f64, y: f64| {
            let binding = my_drag_info.borrow();
            let my_real_drag_info = match binding.as_ref() {
                Some(x) => x,
                None => return,
            };
            let (neg_x_limit, pos_x_limit, neg_y_limit, pos_y_limit) =
                my_draggables.borrow().draggables[my_real_drag_info.index]
                    .borrow()
                    .get_limits();
            my_draggables.borrow_mut().locations[my_real_drag_info.index] = (
                calculate_limits(
                    neg_x_limit,
                    pos_x_limit,
                    my_obj.property("width_request"),
                    my_real_drag_info.start_x + x + my_real_drag_info.relative_x,
                ),
                calculate_limits(
                    neg_y_limit,
                    pos_y_limit,
                    my_obj.property("height_request"),
                    my_real_drag_info.start_y + y + my_real_drag_info.relative_y,
                ),
            );
            my_obj.queue_draw();
        });
        //XXX: There's no connect_drag_end function
        self.obj().add_controller(drag);
    }
}
impl WidgetImpl for DragArea {}
impl DrawingAreaImpl for DragArea {}
