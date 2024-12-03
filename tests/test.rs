use cairo::{Context, Error};
use cairodrag::*;
use gtk4::prelude::*;
use gtk4::{cairo, glib, Application, ApplicationWindow};
const APP_ID: &str = "com.uxugin.cairodrag.test";
struct Thing(f64, f64, f64);
impl Draggable for Thing {
    fn draw(&self, context: &Context, x: f64, y: f64) -> Result<(), Error> {
        context.set_source_rgb(self.0, self.1, self.2);
        context.rectangle(x, y, 100.0, 100.0);
        context.fill()?;
        Ok(())
    }
    fn get_limits(&self) -> (f64, f64, f64, f64) {
        (0.0, 100.0, 0.0, 100.0)
    }
}
struct Circle;
impl Draggable for Circle {
    fn draw(&self, context: &Context, x: f64, y: f64) -> Result<(), Error> {
        context.set_source_rgb(0.0, 1.0, 0.0);
        context.arc(x, y, 50.0, 0.0, 6.29);
        context.fill()?;
        Ok(())
    }
    fn get_limits(&self) -> (f64, f64, f64, f64) {
        (-50.0, 50.0, -50.0, 50.0)
    }
    fn contains(&self, x: f64, y: f64) -> bool {
        (x.powi(2) + y.powi(2)).sqrt() <= 50.0
    }
}
#[test]
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
fn build_ui(app: &Application) {
    let a = Box::new(Thing(1.0, 0.0, 0.0));
    let b = Box::new(Thing(0.0, 0.0, 1.0));
    let c = Box::new(Circle);
    let mut drag_area = DragArea::new(500, 500);
    drag_area.push_box(a, 100.0, 100.0);
    drag_area.push_box(b, 300.0, 100.0);
    drag_area.push_box(c, 250.0, 350.0);
    let window = ApplicationWindow::builder()
        .application(app)
        .child(&drag_area)
        .build();
    window.present();
}
