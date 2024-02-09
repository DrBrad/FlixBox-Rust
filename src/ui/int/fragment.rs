use gtk::{ButtonExt, ContainerExt, CssProvider, CssProviderExt, GtkWindowExt, Image, Orientation, StyleContextExt, WidgetExt, WindowType};

pub trait Fragment<'a> {

    fn new(fragment_root: &'a gtk::Frame) -> Self
        where
            Self: Sized; // To allow calling new only on Sized types

    fn create(fragment_root: &gtk::Frame) -> gtk::Frame {
        let root = gtk::Frame::new(None);
        fragment_root.add(&root);
        root
    }

    fn on_create(&self){
        println!("Created - PARENT");
    }

    fn on_resume(&self){

    }

    fn on_pause(&self){

    }

    fn on_destroy(&self){

    }

    fn get_fragment_root(&self) -> &'a gtk::Frame;

    fn get_root(&self) -> &gtk::Frame;

    /*
    SOMEHOW HAVE THE ABILITY TO SWAP FRAGMENTS...
    */

    /*
    fn start_activity<T: Activity<'a>>(&self) -> T {
        self.on_destroy();
        let window = self.get_window();
        window.remove(self.get_root());
        let new_root = gtk::Box::new(Orientation::Horizontal, 0);
        window.add(&new_root);
        let activity = T::new(window, new_root);
        activity.on_create();
        activity
    }
    */
}