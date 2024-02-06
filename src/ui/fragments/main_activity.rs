use crate::ui::int::activity::Activity;

pub struct MainActivity {
    name: String,
}

impl MainActivity {

    pub fn new(name: String) -> MainActivity {
        MainActivity {
            name
        }
    }
}

impl Activity for MainActivity {

    fn onCreate(&self){
        println!("Created");
    }
}

/*
pub struct Activity {
    window: gtk::Window
}

impl Activity {

    pub fn new(window: &gtk::Window) -> Activity {
        Activity {
            window: window.clone()
        }
    }

    pub fn test(&self){
        println!("Hello World");
        //let parent = gtk::Box::new(Orientation::Horizontal, 2);
        //self.window.add(&parent);

        //let left_nav = gtk::Box::new(Orientation::Vertical, 3);
        //left_nav.set_widget_name("left_nav");
        //left_nav.set_property_width_request(62);
    }

    //onCreate
    //onResume
    //onPause
    //onDestroy


}
*/