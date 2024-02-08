use gtk::prelude::*;
use gtk::{Align, ButtonExt, ContainerExt, CssProvider, CssProviderExt, Fixed, FixedExt, GtkWindowExt, Image, ImageExt, OrientableExt, Orientation, WidgetExt, Window, WindowType};
use gdk_pixbuf::Pixbuf;
use gtk::Align::{Fill, Start};
use crate::ui::int::fragment::Fragment;

pub struct HomeFragment<'a> {
    parent: &'a gtk::Box,
    root_pane: gtk::Box
}

impl<'a> HomeFragment<'a> {

    fn add_header(root_pane: &gtk::ScrolledWindow, list_box: &gtk::Box){
        let parent = gtk::AspectFrame::new(None, 0.5, 0.5, 16.0 / 9.0, false);
        //parent.set_property_height_request(100);
        parent.set_hexpand(true);
        parent.set_vexpand(true);
        //parent.set_valign(Start);
        //parent.set_halign(Fill);
        parent.set_widget_name("header");

        let fixed = gtk::Fixed::new();//(Orientation::Horizontal, 0);
        fixed.set_hexpand(true);
        fixed.set_vexpand(true);
        //fixed.set_valign(Fill);
        //fixed.set_halign(Fill);
        //parent.set_widget_name("fixed");
        parent.add(&fixed);

        let b = gtk::Button::new();
        b.set_hexpand(true);
        b.set_vexpand(true);

        fixed.put(&b, 150, 150);

        list_box.add(&parent);

        let aspect_ratio = 16.0 / 9.0;

        root_pane.connect_size_allocate(move |_, allocation| {
            let width = allocation.width as f64;
            let height = width / aspect_ratio;
            parent.set_size_request(allocation.width, height as i32);
            parent.queue_resize();
        });
    }

    fn add_list(list_box: &gtk::Box){
        let root = gtk::Box::new(Orientation::Horizontal, 0);
        root.set_property_width_request(200);
        root.set_property_height_request(500);

        list_box.add(&root);
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

        let list_box = gtk::Box::new(Orientation::Vertical, 0);
        list_box.set_vexpand(true);
        list_box.set_hexpand(true);
        scrolled_pane.add(&list_box);

        HomeFragment::add_header(&scrolled_pane, &list_box);
        HomeFragment::add_list(&list_box);
        HomeFragment::add_list(&list_box);
        HomeFragment::add_list(&list_box);

        self.root_pane.add(&scrolled_pane);
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
