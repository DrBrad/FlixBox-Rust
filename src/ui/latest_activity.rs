use gtk::{ButtonExt, ContainerExt, CssProvider, CssProviderExt, GtkWindowExt, Image, Orientation, WidgetExt, Window, WindowType};
use gdk_pixbuf::Pixbuf;
use crate::ui::fragments::home_fragment::HomeFragment;
use crate::ui::int::activity::Activity;
use crate::ui::int::fragment::Fragment;

pub struct LatestActivity<'a> {
    window: &'a gtk::Window,
    root_pane: gtk::Frame
}

impl<'a> LatestActivity<'a> {

    fn add_nav_button(left_nav: &gtk::Box, image_path: &str){
        let button = gtk::Button::new();
        button.set_size_request(62, 62);
        //let svg = Image::from_file(image_path);//include_str!(&image_path);
        //button.set_image(Some(&svg));//Image::from_svg(Some(svg), None, None));
        button.set_image(Some(&Image::from_pixbuf(Some(&Pixbuf::from_file_at_scale(image_path, 32, 32, true).unwrap()))));
        left_nav.add(&button);
    }
}

impl<'a> Activity<'a> for LatestActivity<'a> {

    fn new(window: &'a gtk::Window) -> LatestActivity<'a> {
        let root_pane = <LatestActivity<'a> as Activity>::create(window);
        LatestActivity {
            window,
            root_pane
        }
    }

    fn on_create(&self){
        println!("Created");
    }

    fn on_destroy(&self){
        println!("Latest Destroy");
    }

    fn get_window(&self) -> &'a gtk::Window {
        self.window
    }

    fn get_root(&self) -> &gtk::Frame {
        &self.root_pane
    }


    //fn getRootPane(&self) -> &Self::RootPane {
    //    &self.root_pane
    //}
}
