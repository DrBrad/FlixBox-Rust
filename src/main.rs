mod ui;

use std::process::exit;
use gdk::WindowExt;
use gtk::{ButtonExt, ContainerExt, CssProvider, CssProviderExt, GtkWindowExt, Image, Orientation, WidgetExt, WindowType};
use gtk::prelude::BuilderExtManual;
use std::env;

pub use ui::int::activity::Activity;
pub use ui::main_activity::MainActivity;

fn main(){
    env::set_var("GDK_BACKEND", "x11");
    env::set_var("GDK_GL", "always");

    gtk::init().unwrap();

    let window = gtk::Window::new(WindowType::Toplevel);
    init_css(&window);
    window.set_title("FlixBox");
    window.set_default_size(1280, 720);
    window.set_resizable(true);
    window.connect_destroy(|_|exit(0));

    let root = gtk::Box::new(Orientation::Horizontal, 0);
    window.add(&root);

    let activity = MainActivity::new(&window, root);
    activity.on_create();

    window.show_all();
    gtk::main();
}

fn init_css(window: &gtk::Window){
    let css = CssProvider::new();
    css.load_from_path("res/styles/main.css").unwrap();

    let screen = window.get_screen().unwrap();
    gtk::StyleContext::add_provider_for_screen(&screen, &css, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
}
