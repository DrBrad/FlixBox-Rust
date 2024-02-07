use gtk::{ButtonExt, ContainerExt, CssProvider, CssProviderExt, GtkWindowExt, Image, Orientation, WidgetExt, WindowType};

pub trait Activity {

    //type RootPane: WidgetExt;

    fn on_create(&self){
        println!("Created - PARENT");
    }

    fn on_resume(&self){

    }

    fn on_pause(&self){

    }

    fn on_destroy(&self){

    }

    //fn get_root(&self) -> &gtk::Box;

    fn new(root_pane: gtk::Box) -> Self
        where
            Self: Sized; // To allow calling new only on Sized types

    fn start_activity<T: Activity>(&self) -> T {
        self.on_destroy();
        //let window = self.get_root().get_parent();
        let new_root = gtk::Box::new(Orientation::Horizontal, 0);
        //window.add(&new_root);
        T::new(new_root)
    }
}
