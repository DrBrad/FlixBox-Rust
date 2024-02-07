use gtk::{ButtonExt, ContainerExt, CssProvider, CssProviderExt, GtkWindowExt, Image, Orientation, WidgetExt, WindowType};
use gdk_pixbuf::Pixbuf;
use crate::ui::int::activity::Activity;

pub struct MainActivity {
    root_pane: gtk::Box,
}

impl MainActivity {

    fn add_nav_button(left_nav: &gtk::Box, image_path: &str){
        let button = gtk::Button::new();
        button.set_size_request(62, 62);
        //let svg = Image::from_file(image_path);//include_str!(&image_path);
        //button.set_image(Some(&svg));//Image::from_svg(Some(svg), None, None));
        button.set_image(Some(&Image::from_pixbuf(Some(&Pixbuf::from_file_at_scale(image_path, 32, 32, true).unwrap()))));
        left_nav.add(&button);
    }
}

impl Activity for MainActivity {
    
    //type RootPane = gtk::Box;

    fn new(root_pane: gtk::Box) -> MainActivity {
        MainActivity {
            root_pane
        }
    }

    fn on_create(&self){
        println!("Created");

        let parent = gtk::Box::new(Orientation::Horizontal, 2);
        self.root_pane.add(&parent);

        let left_nav = gtk::Box::new(Orientation::Vertical, 3);
        left_nav.set_widget_name("left_nav");
        left_nav.set_property_width_request(62);

        MainActivity::add_nav_button(&left_nav, "res/drawables/ic_home.svg");
        MainActivity::add_nav_button(&left_nav, "res/drawables/ic_latest.svg");
        MainActivity::add_nav_button(&left_nav, "res/drawables/ic_settings.svg");

        parent.add(&left_nav);
    }

    //fn get_root(&self) -> &gtk::Box {
    //    &self.root_pane.clone()
    //}


    //fn getRootPane(&self) -> &Self::RootPane {
    //    &self.root_pane
    //}
}
