//structs and enums in Rust - Data Structures and Algorithms

//derive is a derive macro
#[derive(Debug)]
pub struct Person {

    name: String,
    age: u32,
    location: String,
    children: u32,
    fave_color: Color,

}

impl Person {

    pub fn print(self) -> String {

        format!(      //format! is a macro
 
            "{} is {} years old, lives in {}, and has {} children",

            self.name, self.age, self.location, self.children

        )

    }

}

#[derive(Debug)]
pub enum Color {

    Red,
    Green,
    Blue,

}


fn main() {
    let p = Person {

        name: "Cyndie".to_string(),   //to_string() converts &str to String
        age: 28,
        location: "Nairobi".to_string(),
        children: 2,
        fave_color: Color::Green,
    };

    println!("{:?}", p);
}
