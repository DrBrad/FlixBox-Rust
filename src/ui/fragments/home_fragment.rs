use gtk::{ButtonExt, ContainerExt, CssProvider, CssProviderExt, GtkWindowExt, Image, OrientableExt, Orientation, WidgetExt, Window, WindowType};
use gdk_pixbuf::Pixbuf;
use crate::ui::int::fragment::Fragment;

pub struct HomeFragment<'a> {
    parent: &'a gtk::Box,
    root_pane: gtk::Box
}

impl<'a> HomeFragment<'a> {

    fn add_header(scroll_root: &gtk::Box){
        let root = gtk::Box::new(Orientation::Horizontal, 0);
        root.set_widget_name("splash");
        root.set_property_width_request(200);
        root.set_property_height_request(500);

        scroll_root.add(&root);

    }

    fn add_list(scroll_root: &gtk::Box){
        let root = gtk::Box::new(Orientation::Horizontal, 0);
        root.set_widget_name("splash2");
        root.set_property_width_request(200);
        root.set_property_height_request(500);

        scroll_root.add(&root);
    }
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

        //self.parent.set_widget_name("splash2");
        //self.parent.set_orientation(Orientation::Vertical);

        let scrolled_pane = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        scrolled_pane.set_hexpand(true);

        let scrolled_root = gtk::Box::new(Orientation::Vertical, 0);
        scrolled_pane.add(&scrolled_root);
        //scrolled_window.set_hscrollbar_policy(gtk::PolicyType::Always);
        //scrolled_window.set_vscrollbar_policy(gtk::PolicyType::Automatic);

        /*
        let list_box = gtk::ListBox::new();

        // Simulate a dataset with 100 items
        for i in 0..100 {
            // Create a label for each item
            let label = gtk::Label::new(Some(&format!("Item {}", i)));

            // Add the label to the list box
            list_box.add(&label);
        }

        scrolled_window.add(&list_box);
        */

        HomeFragment::add_header(&scrolled_root);
        HomeFragment::add_list(&scrolled_root);

        self.root_pane.add(&scrolled_pane);

        /*
        let splash = gtk::Box::new(Orientation::Horizontal, 0);
        splash.set_widget_name("splash");
        //splash.set_hexpand(true);
        //splash.set_vexpand(false);
        //splash.set_property_height_request(300);



        let aspect_ratio = 8.0 / 2.0; // Desired aspect ratio (width / height)
        let width = 400; // Desired width
        let height = (width as f64 / aspect_ratio) as i32; // Calculate height based on aspect ratio
        splash.set_size_request(width, height);

        // Connect to the "size-allocate" signal to handle resizing
        splash.connect_size_allocate(move |widget, allocation| {
            let allocation_width = allocation.width;
            let allocation_height = (allocation_width as f64 / aspect_ratio) as i32;
            widget.set_size_request(allocation_width, allocation_height);
        });

        self.root_pane.add(&splash);
        */
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
