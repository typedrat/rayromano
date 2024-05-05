use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Orientation};
use std::env;

const APP_ID: &'static str = "at.typedr.rayromano";

fn main() -> glib::ExitCode {
    tracing_subscriber::fmt().compact().init();

    // Turn off ugly forced GNOME window decorations.
    env::set_var("GTK_CSD", "0");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let vsplit = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .build();

    let hsplit = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .spacing(12)
        .build();

    let label = gtk::Label::builder().label("Resolution").build();
    hsplit.append(&label);

    let drop_down = gtk::DropDown::from_strings(&["256x256", "512x512"]);
    drop_down.set_hexpand(true);
    hsplit.append(&drop_down);

    let button = gtk::Button::builder()
        .label("I'm a placeholder!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    button.connect_clicked(|_| println!("button clicked!"));

    let render_paintable = gtk::gdk::Paintable::new_empty(512, 512);
    let render_view = gtk::Picture::for_paintable(&render_paintable);

    vsplit.append(&hsplit);
    vsplit.append(&render_view);
    vsplit.append(&button);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("rayromano")
        .resizable(false)
        .child(&vsplit)
        .build();

    window.present();
}
