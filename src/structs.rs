// Structs - Used to define the structure of the data.

// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// Tuple Structs
// struct Color(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Constructor
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };

    // c.red = 200;

    // let mut c = Color(255, 0, 0);
    // c.0 = 200;
    // println!("Color: {} {} {}", c.red, c.green, c.blue);
    // println!("Color: {} {} {}", c.0, c.1, c.2);

    println!("Person 1: {}", Person::new("John", "Doe").full_name());

    let mut p = Person::new("Mary", "Smith");
    p.set_last_name("Williams");
    println!("Person 2: {}", p.full_name());

    let (f, l) = p.to_tuple();
    println!("Person 3: {} {}", f, l);
}