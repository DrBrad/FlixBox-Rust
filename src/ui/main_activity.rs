use gtk::{ButtonExt, ContainerExt, CssProvider, CssProviderExt, GtkWindowExt, Image, Orientation, WidgetExt, Window, WindowType};
use gdk_pixbuf::Pixbuf;

use ui::int::activity::Activity;
use ui::int::fragment::Fragment;
use ui::fragments::home_fragment::HomeFragment;
use crate::ui;


pub struct MainActivity<'a> {
    window: &'a gtk::Window,
    root_pane: gtk::Box
}

impl<'a> MainActivity<'a> {

    fn add_nav_button(left_nav: &gtk::Box, image_path: &str){
        let button = gtk::Button::new();
        button.set_size_request(62, 62);
        //let svg = Image::from_file(image_path);//include_str!(&image_path);
        //button.set_image(Some(&svg));//Image::from_svg(Some(svg), None, None));
        button.set_image(Some(&Image::from_pixbuf(Some(&Pixbuf::from_file_at_scale(image_path, 32, 32, true).unwrap()))));
        left_nav.add(&button);
    }
}

impl<'a> Activity<'a> for MainActivity<'a> {

    fn new(window: &'a gtk::Window, root_pane: gtk::Box) -> MainActivity<'a> {
        MainActivity {
            window,
            root_pane
        }
    }

    fn on_create(&self){
        println!("Created");

        let parent = gtk::Box::new(Orientation::Horizontal, 0);
        self.root_pane.add(&parent);

        let left_nav = gtk::Box::new(Orientation::Vertical, 0);
        left_nav.set_widget_name("left_nav");
        left_nav.set_property_width_request(62);

        MainActivity::add_nav_button(&left_nav, "res/drawables/ic_home.svg");
        MainActivity::add_nav_button(&left_nav, "res/drawables/ic_latest.svg");
        MainActivity::add_nav_button(&left_nav, "res/drawables/ic_settings.svg");

        parent.add(&left_nav);

        let fragment_root = gtk::Box::new(Orientation::Horizontal, 0);
        parent.add(&fragment_root);

        let home = HomeFragment::new(&parent, fragment_root);
        home.on_create();
    }

    fn get_window(&self) -> &'a gtk::Window {
        self.window
    }

    fn get_root(&self) -> &gtk::Box {
        &self.root_pane
    }
}
