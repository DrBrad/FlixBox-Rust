use gtk::prelude::*;
use gtk::{Align, ButtonExt, ContainerExt, CssProvider, CssProviderExt, Fixed, FixedExt, GtkWindowExt, Image, ImageExt, OrientableExt, Orientation, PolicyType, WidgetExt, Window, WindowType};
use gdk_pixbuf::Pixbuf;
use gtk::Align::{Fill, Start};
use crate::ui::int::fragment::Fragment;

pub struct HomeFragment<'a> {
    fragment_root: &'a gtk::Frame,
    root_pane: gtk::Frame
}

impl<'a> HomeFragment<'a> {

    fn add_header(scroll_pane: &gtk::ScrolledWindow, list_box: &gtk::Box){
        let aspect_frame = gtk::Frame::new(None);
        aspect_frame.set_vexpand(true);
        aspect_frame.set_hexpand(true);
        aspect_frame.set_widget_name("header");
        list_box.add(&aspect_frame);

        let fixed = gtk::Fixed::new();//(Orientation::Horizontal, 0);
        fixed.set_hexpand(true);
        fixed.set_vexpand(true);
        //fixed.set_valign(Fill);
        //fixed.set_halign(Fill);
        //parent.set_widget_name("fixed");
        aspect_frame.add(&fixed);

        let b = gtk::Button::new();
        b.set_hexpand(true);
        b.set_vexpand(true);

        fixed.put(&b, 150, 150);

        list_box.add(&aspect_frame);


        scroll_pane.connect_size_allocate(move |_, allocation| {
            // Get the new width and height of the widget
            let width = allocation.width as f64;
            let height = width * (9.0 / 16.0); // Maintain a 16:9 aspect ratio

            // Set the size request of the widget
            aspect_frame.set_property_height_request(height as i32);
            aspect_frame.queue_resize();
            //box_layout.queue_compute_expand();
        });
    }

    fn add_list(list_box: &gtk::Box){
        let scrolled_pane = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        scrolled_pane.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Never);

        let genre_box = gtk::Box::new(Orientation::Horizontal, 0);
        scrolled_pane.add(&genre_box);

        for i in 0..20 {
            let cell = gtk::Box::new(Orientation::Horizontal, 0);
            cell.set_property_width_request(200);
            cell.set_property_height_request(150);
            cell.get_style_context().add_class("cell");
            genre_box.add(&cell);
        }

        list_box.add(&scrolled_pane);
    }
}

impl<'a> Fragment<'a> for HomeFragment<'a> {

    fn new(fragment_root: &'a gtk::Frame) -> HomeFragment<'a> {
        let root_pane = <HomeFragment<'a> as Fragment>::create(fragment_root);
        HomeFragment {
            fragment_root,
            root_pane
        }
    }

    fn on_create(&self){
        println!("Created frag");


        let scrolled_pane = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        scrolled_pane.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
        //scrolled_window.set_policy(gtk::Inhibit::Horizontal);
        //self.root_pane.add(&scrolled_window);

        let list_box = gtk::Box::new(Orientation::Vertical, 0);
        scrolled_pane.add(&list_box);

        HomeFragment::add_header(&scrolled_pane, &list_box);
        HomeFragment::add_list(&list_box);
        HomeFragment::add_list(&list_box);
        //HomeFragment::add_list(&list_box);

        for i in 0..20 {
            let label = gtk::Label::new(Some(&format!("Label {}", i)));
            label.set_property_height_request(100);
            list_box.add(&label);
        }

        self.root_pane.add(&scrolled_pane);

        /*
        let flow_box = gtk::FlowBox::new();
        flow_box.set_homogeneous(false); // Optional: Set to true if you want children to have equal sizes

        for i in 0..10 {
            let label = gtk::Label::new(Some(&format!("Label {}", i)));
            flow_box.add(&label);
        }

        self.root_pane.add(&flow_box);
        */


        /*
        let scrolled_pane = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        scrolled_pane.set_vexpand(true);
        scrolled_pane.set_hexpand(true);
        scrolled_pane.set_policy(PolicyType::Never, PolicyType::Automatic); // Allow only vertical scroll
        self.root_pane.add(&scrolled_pane);






        let list_box = gtk::Box::new(Orientation::Vertical, 0);
        list_box.set_vexpand(true);
        list_box.set_hexpand(true);
        scrolled_pane.add(&list_box);*/

        /*
        let aspect = gtk::Frame::new(None);
        aspect.set_hexpand(true);
        aspect.set_vexpand(true);
        aspect.set_widget_name("header");
        list_box.add(&aspect);

        let aspect_ratio = 16.0 / 9.0;

        scrolled_pane.connect_size_allocate(move |_, allocation| {
            let width = allocation.width as f64;
            let height = width / aspect_ratio;
            aspect.set_size_request(allocation.width, height as i32);
            aspect.queue_resize();
        });
        */

        //self.parent.set_widget_name("splash2");
        //self.parent.set_orientation(Orientation::Vertical);

        /*

        let scrolled_pane = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        scrolled_pane.set_vexpand(true);
        scrolled_pane.set_hexpand(true);

        let list_box = gtk::Box::new(Orientation::Vertical, 0);
        list_box.set_vexpand(true);
        list_box.set_hexpand(true);
        scrolled_pane.add(&list_box);


        self.root_pane.add(&scrolled_pane);
        */
    }

    fn get_fragment_root(&self) -> &'a gtk::Frame {
        self.fragment_root
    }

    fn get_root(&self) -> &gtk::Frame {
        &self.root_pane
    }
}
