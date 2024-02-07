use gtk::Orientation;



pub trait Activity {

    fn onCreate(&self){
        println!("Created - PARENT");
    }

    fn onResume(&self){

    }

    fn onPause(&self){

    }

    fn onDestroy(&self){

    }
}

/*
impl Activity {

    pub fn new(window: &gtk::Window){
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