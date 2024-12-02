use cairodrag::*;
use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow};
const APP_ID: &str = "com.uxugin.cairodrag.test";
#[test]
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
fn build_ui(app: &Application) {
    let drag_area = DragArea::new(500, 500);
    let window = ApplicationWindow::builder()
        .application(app)
        .child(&drag_area)
        .build();
    window.present();
    //panic!("LOOK AT ME LOOK AT ME LOOK AT ME LOOK AT ME LOOK AT ME LOOK AT ME LOOK AT ME LOOK AT ME");
}
