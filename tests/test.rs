use cairo::{Context, Error};
use cairodrag::*;
use gtk4::prelude::*;
use gtk4::{cairo, glib, Application, ApplicationWindow};
const APP_ID: &str = "com.uxugin.cairodrag.test";
struct Thing;
impl Draggable for Thing {
    fn draw(&self, context: &Context, x: f64, y: f64) -> Result<(), Error> {
        context.set_source_rgb(0.5, 0.5, 0.5);
        context.rectangle(x, y, 100.0, 100.0);
        context.fill()?;
        Ok(())
    }
}
#[test]
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
fn build_ui(app: &Application) {
    let thing = Box::new(Thing);
    let mut drag_area = DragArea::new(500, 500);
    drag_area.push_box(thing, 100.0, 100.0);
    let window = ApplicationWindow::builder()
        .application(app)
        .child(&drag_area)
        .build();
    window.present();
}
