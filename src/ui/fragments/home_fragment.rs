use gtk::{Align, ButtonExt, ContainerExt, CssProvider, CssProviderExt, GtkWindowExt, Image, ImageExt, OrientableExt, Orientation, WidgetExt, Window, WindowType};
use gdk_pixbuf::Pixbuf;
use crate::ui::int::fragment::Fragment;

pub struct HomeFragment<'a> {
    parent: &'a gtk::Box,
    root_pane: gtk::Box
}

impl<'a> HomeFragment<'a> {

    fn add_header(root_pane: &gtk::Box, scroll_root: &gtk::Box){
        //let root = gtk::Box::new(Orientation::Horizontal, 0);
        //root.set_hexpand(true);
        //root.set_widget_name("header");

        //let root = gtk::AspectFrame::new(None, 0.5, 0.5, 16.0 / 9.0, true);
        //root.set_valign(Align::Start);
        //root.set_hexpand(true);
        //root.set_vexpand(true);

        let root = gtk::Box::new(Orientation::Horizontal, 0);
        root.set_hexpand(true);
        root.set_vexpand(true);
        root.set_widget_name("header");
        //root.add(&root);
        scroll_root.add(&root);

        let aspect_ratio = 16.0 / 9.0;

        root_pane.connect_size_allocate(move |_, allocation| {
            println!("{}, {}", allocation.width, allocation.height);
            let width = allocation.width as f64;
            let height = width / aspect_ratio;
            root.set_size_request(allocation.width, height as i32);
        });

        /*
        let pixbuf = Pixbuf::from_file("/var/www/images/landscape/d04f148795a31c4c0723e49fe635ec5e297aba89a76ae73060eb1b031609f7fd.jpg").unwrap();
        //let scaled_pixbuf = pixbuf.scale_simple(960, 540, gdk_pixbuf::InterpType::Hyper).unwrap(); // Adjust the dimensions as needed

        //image.set_from_pixbuf(Some(&pixbuf));
        //image.set_from_pixbuf(Some(&Pixbuf::from_file("/var/www/images/landscape/d04f148795a31c4c0723e49fe635ec5e297aba89a76ae73060eb1b031609f7fd.jpg").unwrap()));
        root.add(&image);

        root.connect_size_allocate(move |_, allocation| {
            println!("{}, {}", image.get_allocated_width(), image.get_allocated_height());

            let width = allocation.width as i32;
            let height = allocation.height as i32;

            // Calculate the scaling dimensions while preserving the aspect ratio
            let (scaled_width, scaled_height) = if width * pixbuf.get_height() > height * pixbuf.get_width() {
                let new_height = height;
                let new_width = (new_height * pixbuf.get_width()) / pixbuf.get_height();
                (new_width, new_height)
            } else {
                let new_width = width;
                let new_height = (new_width * pixbuf.get_height()) / pixbuf.get_width();
                (new_width, new_height)
            };

            let scaled_pixbuf = pixbuf.scale_simple(scaled_width-2, scaled_height-2, gdk_pixbuf::InterpType::Hyper).unwrap();
            image.set_from_pixbuf(Some(&scaled_pixbuf));
        });
        */

        /*
        image.connect_size_allocate(move |_, allocation| {
            let pixbufa = pixbuf.clone();
            // Get the new width and height of the window
            let width = allocation.width;
            let height = allocation.height;

            // Scale the image to fit the new window size
            let new_pixbuf = pixbufa.scale_simple(width, height, gdk_pixbuf::InterpType::Hyper).unwrap();
            image.set_from_pixbuf(Some(&new_pixbuf));
        });
        */

        //scroll_root.add(&root);

    }

    fn add_list(scroll_root: &gtk::Box){
        let root = gtk::Box::new(Orientation::Horizontal, 0);
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
        scrolled_pane.set_vexpand(true);
        scrolled_pane.set_hexpand(true);

        let scrolled_root = gtk::Box::new(Orientation::Vertical, 0);
        //scrolled_root.set_valign(Align::Start);
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

        HomeFragment::add_header(&self.root_pane, &scrolled_root);
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
