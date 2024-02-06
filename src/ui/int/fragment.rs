pub struct Activity {
    field1: i32,
    field2: String,
}

// Implement methods for the MyClass struct
impl Activity {
    // Constructor method to create a new instance of MyClass
    pub fn new(field1: i32, field2: String) -> Activity {
        Activity { field1, field2 }
    }

    // Method to perform some action
    pub fn do_something(&self) {
        println!("Field 1: {}", self.field1);
        println!("Field 2: {}", self.field2);
    }
}