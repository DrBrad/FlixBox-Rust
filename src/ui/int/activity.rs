use gtk::{ButtonExt, ContainerExt, CssProvider, CssProviderExt, GtkWindowExt, Image, Orientation, StyleContextExt, WidgetExt, WindowType};

pub trait Activity<'a> {

    fn new(window: &'a gtk::Window) -> Self
        where
            Self: Sized; // To allow calling new only on Sized types

    fn create(window: &gtk::Window) -> gtk::Frame {
        let root = gtk::Frame::new(None);
        window.add(&root);
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

    fn get_window(&self) -> &'a gtk::Window;

    fn get_root(&self) -> &gtk::Frame;

    fn start_activity<T: Activity<'a>>(&self) -> T {
        self.on_destroy();
        let window = self.get_window();
        window.remove(self.get_root());
        let activity = T::new(window);
        activity.on_create();
        activity
    }
}
