use gtk::{ButtonExt, ContainerExt, CssProvider, CssProviderExt, GtkWindowExt, Image, Orientation, WidgetExt, Window, WindowType};
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
