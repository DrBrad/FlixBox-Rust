use gtk::{ButtonExt, ContainerExt, CssProvider, CssProviderExt, GtkWindowExt, Image, Orientation, StyleContextExt, WidgetExt, Window, WindowType};
use gdk_pixbuf::Pixbuf;

use ui::int::activity::Activity;
use ui::int::fragment::Fragment;
use ui::fragments::home_fragment::HomeFragment;
use crate::ui;
use crate::ui::latest_activity::LatestActivity;


pub struct MainActivity<'a> {
    window: &'a gtk::Window,
    root_pane: gtk::Frame
}

impl<'a> MainActivity<'a> {

    fn add_nav_button(left_nav: &gtk::Box, image_path: &str) -> gtk::Button {
        let button = gtk::Button::new();
        button.set_size_request(62, 62);
        //let svg = Image::from_file(image_path);//include_str!(&image_path);
        //button.set_image(Some(&svg));//Image::from_svg(Some(svg), None, None));
        button.set_image(Some(&Image::from_pixbuf(Some(&Pixbuf::from_file_at_scale(image_path, 32, 32, true).unwrap()))));
        left_nav.add(&button);
        button
    }
}

impl<'a> Activity<'a> for MainActivity<'a> {

    fn new(window: &'a gtk::Window) -> MainActivity<'a> {
        let root_pane = <MainActivity<'a> as Activity>::create(window);
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

        let home_button = MainActivity::add_nav_button(&left_nav, "res/drawables/ic_home.svg");
        home_button.get_style_context().add_class("selected");
        let latest_button = MainActivity::add_nav_button(&left_nav, "res/drawables/ic_latest.svg");
        let settings_button = MainActivity::add_nav_button(&left_nav, "res/drawables/ic_settings.svg");

        parent.add(&left_nav);

        let fragment_root = gtk::Frame::new(None);
        parent.add(&fragment_root);

        let home = HomeFragment::new(&fragment_root);
        home.on_create();
    }

    fn get_window(&self) -> &'a gtk::Window {
        self.window
    }

    fn get_root(&self) -> &gtk::Frame {
        &self.root_pane
    }
}
