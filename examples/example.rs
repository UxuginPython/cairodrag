// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
use cairo::{Context, Error};
use cairodrag::*;
use gtk4::prelude::*;
use gtk4::{cairo, glib, Application, ApplicationWindow};
use std::cell::Cell;
const APP_ID: &str = "com.uxugin.cairodrag.example";
struct Square {
    r: f64,
    g: f64,
    b: f64,
    retain_flag: Cell<bool>,
}
impl Square {
    fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            r: r,
            g: g,
            b: b,
            retain_flag: Cell::new(true),
        }
    }
}
impl Draggable for Square {
    fn draw(&self, context: &Context, x: f64, y: f64) -> Result<(), Error> {
        context.set_source_rgb(self.r, self.g, self.b);
        context.rectangle(x, y, 100.0, 100.0);
        context.fill()?;
        Ok(())
    }
    fn get_limits(&self) -> (f64, f64, f64, f64) {
        (0.0, 100.0, 0.0, 100.0)
    }
    fn retain(&self) -> bool {
        self.retain_flag.get()
    }
    fn on_middle_click(&self) {
        println!("square middle clicked - removing");
        self.retain_flag.set(false);
    }
}
struct Circle(f64, f64, f64);
impl Draggable for Circle {
    fn draw(&self, context: &Context, x: f64, y: f64) -> Result<(), Error> {
        context.set_source_rgb(self.0, self.1, self.2);
        context.arc(x, y, 50.0, 0.0, 6.29);
        context.fill()?;
        Ok(())
    }
    fn get_limits(&self) -> (f64, f64, f64, f64) {
        (50.0, 50.0, 50.0, 50.0)
    }
    fn contains(&self, x: f64, y: f64) -> bool {
        (x.powi(2) + y.powi(2)).sqrt() <= 50.0
    }
    fn on_double_click(&self) {
        println!("circle double clicked");
    }
    fn on_middle_click(&self) {
        println!("circle middle clicked");
    }
    fn on_right_click(&self) {
        println!("circle right clicked");
    }
}
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
fn build_ui(app: &Application) {
    let a = Box::new(Square::new(1.0, 0.0, 0.0));
    let b = Box::new(Square::new(0.0, 0.0, 1.0));
    let c = Box::new(Circle(0.0, 1.0, 0.0));
    let drag_area = DragArea::new(500, 500);
    drag_area.push_box(a, 100.0, 100.0);
    drag_area.push_box(b, 300.0, 100.0);
    drag_area.push_box(c, 250.0, 350.0);
    let window = ApplicationWindow::builder()
        .application(app)
        .child(&drag_area)
        .build();
    window.present();
}
