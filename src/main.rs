use std::process::exit;
use gtk::{ButtonExt, ContainerExt, CssProvider, CssProviderExt, GtkWindowExt, Image, Orientation, WidgetExt, WindowType};

mod ui {
    pub mod int {
        pub mod activity;
    }
    pub mod fragments {
        pub mod main_activity;
    }
}

use ui::fragments::main_activity::MainActivity;
use ui::int::activity::Activity;

fn main(){
    gtk::init().unwrap();

    let window = gtk::Window::new(WindowType::Toplevel);
    init_css(&window);
    window.set_title("FlixBox");
    window.set_default_size(1280, 720);
    window.set_resizable(true);
    window.connect_destroy(|_|exit(0));

    //NOT SURE HOW TO USE JUST THE FUCKING WINDOW SO THIS WILL DO...
    //let window_root = gtk::Box::new(Orientation::Horizontal, 0);
    //window.add(&window_root);

    //CAN WE SOMEHOW MAKE THIS A FUNCTION...
    let root = gtk::Box::new(Orientation::Horizontal, 0);
    window.add(&root);

    let activity = MainActivity::new(root);
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
