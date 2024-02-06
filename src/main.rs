use std::process::exit;
use gtk::{ButtonExt, ContainerExt, CssProvider, CssProviderExt, GtkWindowExt, Image, Orientation, WidgetExt, WindowType};
use gdk_pixbuf::Pixbuf;

mod ui {

    pub mod int {
        pub mod activity;
    }
}

// Use the MyClass struct from the my_class module
use ui::int::activity::Activity;

fn main(){




    gtk::init().unwrap();

    let window = gtk::Window::new(WindowType::Toplevel);
    //init_css(&window);
    window.set_title("FlixBox");
    window.set_default_size(1280, 720);
    window.set_resizable(true);
    window.connect_destroy(|_|exit(0));


    let activity = Activity::new(window);


    window.show_all();


    /*
    gtk::init().unwrap();
    init_window();
    gtk::main();
    */
}

/*
fn init_css(window: &gtk::Window){
    let css = CssProvider::new();
    css.load_from_path("res/styles/main.css").unwrap();

    let screen = window.get_screen().unwrap();
    gtk::StyleContext::add_provider_for_screen(&screen, &css, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
}

fn init_window(){
    let window = gtk::Window::new(WindowType::Toplevel);
    init_css(&window);
    window.set_title("FlixBox");
    window.set_default_size(1280, 720);
    window.set_resizable(true);
    window.connect_destroy(|_|exit(0));

    //let icon = Some(&Image::from_pixbuf(Some(&Pixbuf::from_file_at_scale("res/drawables/ic_launcher.svg", 32, 32, true).unwrap())));
    //let icon: Pixbuf = Pixbuf::from_resource("../res/drawables/ic_launcher.svg").unwrap();
    //window.set_icon(Option::from(&icon));




    let parent = gtk::Box::new(Orientation::Horizontal, 2);
    window.add(&parent);

    let left_nav = gtk::Box::new(Orientation::Vertical, 3);
    left_nav.set_widget_name("left_nav");
    left_nav.set_property_width_request(62);

    add_nav_button(&left_nav, "res/drawables/ic_home.svg");
    add_nav_button(&left_nav, "res/drawables/ic_latest.svg");
    add_nav_button(&left_nav, "res/drawables/ic_settings.svg");

    parent.add(&left_nav);




    window.show_all();
}

fn add_nav_button(left_nav: &gtk::Box, image_path: &str){
    let button = gtk::Button::new();
    button.set_size_request(62, 62);
    //let svg = Image::from_file(image_path);//include_str!(&image_path);
    //button.set_image(Some(&svg));//Image::from_svg(Some(svg), None, None));
    button.set_image(Some(&Image::from_pixbuf(Some(&Pixbuf::from_file_at_scale(image_path, 32, 32, true).unwrap()))));
    left_nav.add(&button);
}
*/