use gtk::{ButtonExt, ContainerExt, CssProvider, CssProviderExt, GtkWindowExt, Image, OrientableExt, Orientation, WidgetExt, Window, WindowType};
use gdk_pixbuf::Pixbuf;
use crate::ui::int::fragment::Fragment;

pub struct HomeFragment<'a> {
    parent: &'a gtk::Box,
    root_pane: gtk::Box
}

impl<'a> HomeFragment<'a> {

}

impl<'a> Fragment<'a> for HomeFragment<'a> {

    fn new(parent: &'a gtk::Box, root_pane: gtk::Box) -> HomeFragment<'a> {
        HomeFragment {
            parent,
            root_pane
        }
    }

    fn on_create(&self){
        println!("Created frag");

        //self.parent.set_widget_name("splash");
        //self.parent.set_orientation(Orientation::Vertical);

        let splash = gtk::Box::new(Orientation::Horizontal, 0);
        splash.set_widget_name("splash");
        splash.set_hexpand(true);
        splash.set_vexpand(true);
        //splash.set_property_height_request(300);

        /*
        let aspect_ratio = 16.0 / 9.0; // Desired aspect ratio (width / height)
        let width = 400; // Desired width
        let height = (width as f64 / aspect_ratio) as i32; // Calculate height based on aspect ratio
        splash.set_size_request(width, height);

        // Connect to the "size-allocate" signal to handle resizing
        splash.connect_size_allocate(move |widget, allocation| {
            let allocation_width = allocation.width;
            let allocation_height = (allocation_width as f64 / aspect_ratio) as i32;
            widget.set_size_request(allocation_width, allocation_height);
        });
        */

        self.parent.add(&splash);
    }

    fn get_parent(&self) -> &'a gtk::Box {
        self.parent
    }

    fn get_root(&self) -> &gtk::Box {
        &self.root_pane
    }


    //fn getRootPane(&self) -> &Self::RootPane {
    //    &self.root_pane
    //}
}
