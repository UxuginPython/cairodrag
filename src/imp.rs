// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
use crate::{Context, Draggable};
use gtk4::{glib, prelude::*, subclass::prelude::*, DrawingArea, GestureClick, GestureDrag};
use std::cell::{Cell, RefCell};
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
            Self::Box(box_) => ReferenceBorrow::NormalReference(box_),
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
struct DraggableAndCoordinates {
    draggable: Reference<dyn Draggable>,
    x: f64,
    y: f64,
}
impl DraggableAndCoordinates {
    fn borrow(&self) -> DraggableBorrowAndCoordinates {
        DraggableBorrowAndCoordinates {
            draggable: self.draggable.borrow(),
            x: self.x,
            y: self.y,
        }
    }
}
struct DraggableSetHolder {
    draggables_and_locs: Vec<DraggableAndCoordinates>,
}
impl DraggableSetHolder {
    fn new() -> Self {
        Self {
            draggables_and_locs: Vec::new(),
        }
    }
    fn push(&mut self, item: Reference<dyn Draggable>, x: f64, y: f64) {
        self.draggables_and_locs.push(DraggableAndCoordinates {
            draggable: item,
            x: x,
            y: y,
        })
    }
    fn iter(&self) -> DraggableSetHolderIterator<'_> {
        let len = self.draggables_and_locs.len();
        let index_back = if len >= 1 { len - 1 } else { 0 };
        DraggableSetHolderIterator {
            holder: self, //This is a reference
            index_start: 0,
            index_back: index_back,
        }
    }
    fn move_to_end(&mut self, index: usize) -> usize {
        let element = self.draggables_and_locs.remove(index);
        self.draggables_and_locs.push(element);
        self.draggables_and_locs.len() - 1
    }
    fn retain(&mut self) {
        self.draggables_and_locs
            .retain(|draggable_and_coords| draggable_and_coords.draggable.borrow().retain());
    }
}
struct DraggableBorrowAndCoordinates<'a> {
    draggable: ReferenceBorrow<'a, dyn Draggable>,
    x: f64,
    y: f64,
}
struct DraggableSetHolderIterator<'a> {
    holder: &'a DraggableSetHolder,
    index_start: usize,
    index_back: usize,
}
impl<'a> Iterator for DraggableSetHolderIterator<'a> {
    type Item = DraggableBorrowAndCoordinates<'a>;
    fn next(&mut self) -> Option<DraggableBorrowAndCoordinates<'a>> {
        if self.index_start >= self.holder.draggables_and_locs.len()
            || self.index_start > self.index_back
        {
            return None;
        }
        let output = self.holder.draggables_and_locs[self.index_start].borrow();
        self.index_start += 1;
        Some(output)
    }
}
impl<'a> DoubleEndedIterator for DraggableSetHolderIterator<'a> {
    fn next_back(&mut self) -> Option<DraggableBorrowAndCoordinates<'a>> {
        //usize type keeps it from going below zero
        if self.index_back < self.index_start {
            return None;
        }
        let output = self.holder.draggables_and_locs[self.index_back].borrow();
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
    scrollable: Rc<Cell<bool>>,
    scrolling: Rc<Cell<bool>>,
    translate: Rc<Cell<(f64, f64)>>,
    drag_translate: Rc<Cell<(f64, f64)>>,
    pre_draw_func: Rc<RefCell<Option<Box<dyn FnMut(&DrawingArea, &Context, i32, i32)>>>>,
    post_draw_func: Rc<RefCell<Option<Box<dyn FnMut(&DrawingArea, &Context, i32, i32)>>>>,
}
impl DragArea {
    pub fn new() -> Self {
        let draggables = Rc::new(RefCell::new(DraggableSetHolder::new()));
        Self {
            draggables: draggables,
            drag_info: Rc::new(RefCell::new(None)),
            scrollable: Rc::new(Cell::new(false)),
            scrolling: Rc::new(Cell::new(false)),
            translate: Rc::new(Cell::new((0.0, 0.0))),
            drag_translate: Rc::new(Cell::new((0.0, 0.0))),
            pre_draw_func: Rc::new(RefCell::new(None)),
            post_draw_func: Rc::new(RefCell::new(None)),
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
    pub fn get_scroll_location(&self) -> (f64, f64) {
        let (trans_x, trans_y) = self.translate.get();
        let (drag_trans_x, drag_trans_y) = self.drag_translate.get();
        (trans_x + drag_trans_x, trans_y + drag_trans_y)
    }
    pub fn set_scrollable(&self, scrollable: bool) {
        self.scrollable.set(scrollable);
    }
    pub fn set_pre_draw_func(
        &self,
        pre_draw_func: Box<impl FnMut(&DrawingArea, &Context, i32, i32) + 'static>,
    ) {
        *self.pre_draw_func.borrow_mut() =
            Some(pre_draw_func as Box<dyn FnMut(&DrawingArea, &Context, i32, i32)>);
    }
    pub fn unset_pre_draw_func(&self) {
        *self.pre_draw_func.borrow_mut() = None;
    }
    pub fn set_post_draw_func(
        &self,
        post_draw_func: Box<impl FnMut(&DrawingArea, &Context, i32, i32) + 'static>,
    ) {
        *self.post_draw_func.borrow_mut() =
            Some(post_draw_func as Box<dyn FnMut(&DrawingArea, &Context, i32, i32)>);
    }
    pub fn unset_post_draw_func(&self) {
        *self.post_draw_func.borrow_mut() = None;
    }
}
impl Default for DragArea {
    fn default() -> Self {
        Self {
            draggables: Rc::new(RefCell::new(DraggableSetHolder::new())),
            drag_info: Rc::new(RefCell::new(None)),
            scrollable: Rc::new(Cell::new(false)),
            scrolling: Rc::new(Cell::new(false)),
            translate: Rc::new(Cell::new((0.0, 0.0))),
            drag_translate: Rc::new(Cell::new((0.0, 0.0))),
            pre_draw_func: Rc::new(RefCell::new(None)),
            post_draw_func: Rc::new(RefCell::new(None)),
        }
    }
}
#[glib::object_subclass]
impl ObjectSubclass for DragArea {
    const NAME: &'static str = "CairoDragDragArea";
    type Type = super::DragArea;
    type ParentType = DrawingArea;
}
fn calculate_limits(
    neg_limit: f64,
    pos_limit: f64,
    area_size: i32, //width or height, whichever we're calculating
    scrollable: bool,
    desired_coord: f64,
) -> f64 {
    if !scrollable {
        if desired_coord < neg_limit {
            return neg_limit;
        }
        if desired_coord > area_size as f64 - pos_limit {
            return area_size as f64 - pos_limit;
        }
    }
    desired_coord
}
impl ObjectImpl for DragArea {
    fn constructed(&self) {
        self.parent_constructed();
        let my_draggables = self.draggables.clone();
        let my_translate = self.translate.clone();
        let my_drag_translate = self.drag_translate.clone();
        let my_pre_draw_func = self.pre_draw_func.clone();
        let my_post_draw_func = self.post_draw_func.clone();
        self.obj()
            .set_draw_func(move |drawing_area, context, width, height| {
                match &mut *my_pre_draw_func.borrow_mut() {
                    //This works because drawing_area and context are references.
                    Some(func) => (*func)(drawing_area, context, width, height),
                    None => (),
                }
                my_draggables.borrow_mut().retain();
                for i in my_draggables.borrow().iter() {
                    let (trans_x, trans_y) = my_translate.get();
                    let (drag_trans_x, drag_trans_y) = my_drag_translate.get();
                    let x = i.x + trans_x + drag_trans_x;
                    let y = i.y + trans_y + drag_trans_y;
                    i.draggable.draw(&context, x, y).unwrap();
                }
                match &mut *my_post_draw_func.borrow_mut() {
                    Some(func) => (*func)(drawing_area, context, width, height),
                    None => (),
                }
            });
        let drag = GestureDrag::new();
        let my_draggables = self.draggables.clone();
        let my_drag_info = self.drag_info.clone();
        let my_obj = self.obj().clone();
        let my_scrolling = self.scrolling.clone();
        let my_translate = self.translate.clone();
        drag.connect_drag_begin(move |_gesture: &GestureDrag, x: f64, y: f64| {
            //drag_translate is always (0.0, 0.0) when we're not actively dragging, which we're not
            //when the drag begin function is called.
            let (trans_x, trans_y) = my_translate.get();
            let mut new_drag_info = None;
            let mut scrolling = true;
            for (i, draggable_and_coords) in my_draggables.borrow().iter().enumerate() {
                if draggable_and_coords.draggable.contains(
                    x - trans_x - draggable_and_coords.x,
                    y - trans_y - draggable_and_coords.y,
                ) {
                    new_drag_info = Some(DragInfo {
                        start_x: x,
                        start_y: y,
                        index: i,
                        relative_x: draggable_and_coords.x - x,
                        relative_y: draggable_and_coords.y - y,
                    })
                }
                if !draggable_and_coords.draggable.can_scroll(
                    x - trans_x - draggable_and_coords.x,
                    y - trans_y - draggable_and_coords.y,
                ) {
                    scrolling = false;
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
            my_scrolling.set(scrolling);
            my_obj.queue_draw();
        });
        let my_draggables = self.draggables.clone();
        let my_drag_info = self.drag_info.clone();
        let my_obj = self.obj().clone();
        let my_scrollable = self.scrollable.clone();
        let my_scrolling = self.scrolling.clone();
        let my_drag_translate = self.drag_translate.clone();
        drag.connect_drag_update(move |_gesture: &GestureDrag, x: f64, y: f64| {
            let scrollable = my_scrollable.get();
            let scrolling = my_scrolling.get();
            let binding = my_drag_info.borrow();
            let my_real_drag_info = match binding.as_ref() {
                Some(x) => x,
                None => {
                    if scrollable && scrolling {
                        my_drag_translate.set((x, y));
                        my_obj.queue_draw();
                    }
                    return;
                }
            };
            let (neg_x_limit, pos_x_limit, neg_y_limit, pos_y_limit) =
                my_draggables.borrow().draggables_and_locs[my_real_drag_info.index]
                    .draggable
                    .borrow()
                    .get_limits();
            my_draggables.borrow_mut().draggables_and_locs[my_real_drag_info.index].x =
                calculate_limits(
                    neg_x_limit,
                    pos_x_limit,
                    my_obj.property("width_request"),
                    scrollable,
                    my_real_drag_info.start_x + x + my_real_drag_info.relative_x,
                );
            my_draggables.borrow_mut().draggables_and_locs[my_real_drag_info.index].y =
                calculate_limits(
                    neg_y_limit,
                    pos_y_limit,
                    my_obj.property("height_request"),
                    scrollable,
                    my_real_drag_info.start_y + y + my_real_drag_info.relative_y,
                );
            my_obj.queue_draw();
        });
        let my_obj = self.obj().clone();
        let my_translate = self.translate.clone();
        let my_drag_translate = self.drag_translate.clone();
        //XXX: This does not update the position of anything
        drag.connect_drag_end(move |_gesture: &GestureDrag, _x: f64, _y: f64| {
            let (old_trans_x, old_trans_y) = my_translate.get();
            let (drag_trans_x, drag_trans_y) = my_drag_translate.get();
            //let (drag_trans_x, drag_trans_y) = (x, y);
            let new_trans = (old_trans_x + drag_trans_x, old_trans_y + drag_trans_y);
            my_translate.set(new_trans);
            my_drag_translate.set((0.0, 0.0));
            my_obj.queue_draw();
        });
        self.obj().add_controller(drag);
        enum ClickType {
            Double,
            Middle,
            Right,
        }
        let my_draggables = self.draggables.clone();
        let my_translate = self.translate.clone();
        let my_obj = self.obj().clone();
        let click = move |click_type: ClickType, x: f64, y: f64| {
            let (trans_x, trans_y) = my_translate.get();
            for draggable_and_coords in my_draggables.borrow().iter() {
                if draggable_and_coords.draggable.contains(
                    x - trans_x - draggable_and_coords.x,
                    y - trans_y - draggable_and_coords.y,
                ) {
                    match click_type {
                        ClickType::Double => draggable_and_coords.draggable.on_double_click(),
                        ClickType::Middle => draggable_and_coords.draggable.on_middle_click(),
                        ClickType::Right => draggable_and_coords.draggable.on_right_click(),
                    }
                }
            }
            my_obj.queue_draw();
        };
        let left_click = GestureClick::new();
        left_click.set_button(1);
        let my_click = click.clone();
        left_click.connect_pressed(move |_, clicks, x, y| {
            if clicks == 2 {
                my_click(ClickType::Double, x, y);
            }
        });
        self.obj().add_controller(left_click);
        let middle_click = GestureClick::new();
        middle_click.set_button(2);
        let my_click = click.clone();
        middle_click.connect_pressed(move |_, clicks, x, y| {
            if clicks == 1 {
                my_click(ClickType::Middle, x, y);
            }
        });
        self.obj().add_controller(middle_click);
        let right_click = GestureClick::new();
        right_click.set_button(3);
        right_click.connect_pressed(move |_, clicks, x, y| {
            if clicks == 1 {
                click(ClickType::Right, x, y);
            }
        });
        self.obj().add_controller(right_click);
    }
}
impl WidgetImpl for DragArea {}
impl DrawingAreaImpl for DragArea {}
