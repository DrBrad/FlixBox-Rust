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

    fn add_header(root_pane: &gtk::ScrolledWindow, list_box: &gtk::Frame){

        let parent = gtk::AspectFrame::new(None, 0.5, 0.5, 16.0 / 9.0, false);
        //parent.set_property_height_request(100);
        parent.set_hexpand(true);
        parent.set_vexpand(true);
        parent.set_valign(Fill);
        parent.set_halign(Fill);
        parent.set_widget_name("header");

        /*
        let aspect = gtk::Box::new(Orientation::Horizontal, 0);
        aspect.set_hexpand(true);
        aspect.set_vexpand(true);
        aspect.set_widget_name("header");
        parent.add(&aspect);

        let fixed = gtk::Fixed::new();//(Orientation::Horizontal, 0);
        fixed.set_hexpand(true);
        fixed.set_vexpand(true);
        //fixed.set_valign(Fill);
        //fixed.set_halign(Fill);
        //parent.set_widget_name("fixed");
        aspect.add(&fixed);

        let b = gtk::Button::new();
        b.set_hexpand(true);
        b.set_vexpand(true);

        fixed.put(&b, 150, 150);

        list_box.add(&parent);

        let aspect_ratio = 16.0 / 9.0;
        */

        /*
        root_pane.connect_size_allocate(move |_, allocation| {
            let width = allocation.width as f64;
            let height = width / aspect_ratio;
            parent.set_size_request(allocation.width, height as i32);
            parent.queue_resize();
        });
        */


        list_box.add(&parent);
    }

    fn add_list(list_box: &gtk::Box){
        let root = gtk::Box::new(Orientation::Horizontal, 0);
        root.set_property_width_request(200);
        root.set_property_height_request(500);

        list_box.add(&root);
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
        scrolled_pane.set_vexpand(true);
        scrolled_pane.set_hexpand(true);
        scrolled_pane.set_policy(PolicyType::Never, PolicyType::Automatic); // Allow only vertical scroll
        self.root_pane.add(&scrolled_pane);






        let list_box = gtk::Box::new(Orientation::Vertical, 0);
        list_box.set_vexpand(true);
        list_box.set_hexpand(true);
        scrolled_pane.add(&list_box);

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

        //HomeFragment::add_header(&scrolled_pane, &list_box);
        //HomeFragment::add_list(&list_box);
        //HomeFragment::add_list(&list_box);
        //HomeFragment::add_list(&list_box);

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
