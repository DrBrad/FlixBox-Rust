use gtk::{ButtonExt, ContainerExt, CssProvider, CssProviderExt, GtkWindowExt, Image, Orientation, WidgetExt, WindowType};

pub trait Fragment<'a> {

    fn new(parent: &'a gtk::Box, root_pane: gtk::Box) -> Self
        where
            Self: Sized; // To allow calling new only on Sized types

    fn on_create(&self){
        println!("Created - PARENT");
    }

    fn on_resume(&self){

    }

    fn on_pause(&self){

    }

    fn on_destroy(&self){

    }

    fn get_parent(&self) -> &'a gtk::Box;

    fn get_root(&self) -> &gtk::Box;

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