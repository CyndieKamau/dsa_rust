//Simple exercises for building on pattern matching in enums in Rust

//1. Define an enum `Color` with three variants, `Red`, `Green`, `Blue`.
//Use `match` to write a function `to_hex` that takes a `Color` and returns its hex rep as strings
//Use `if let` to write a function `is_red` that returns `true` if the color is Red, `false` otherwise

//2. Define an enum `Animal` with three variants;
// `Dog` with a name,
// `Cat` with age,
//`Bird`
//use `match` -> function `describe` that prints out dog's name, cat's age, or "it's a bird!" for bird
//use `if let` to check if animal is a cat. If not, print "Not a cat!"

//3. Define an enum `Transport` with four variants;
// `Car` -> speed(i32), brand(String),
// `Plane` -> altitude(i32),
// `Boat` -> no of passengers(i32)
//use match to implement method `info` on all four variants of the enum
//use if let -> method `is_plane` to check if `Transport` is plane or not  

#[derive(Debug)]
enum Color {

    Red,
    Green,
    Blue,
}

impl Color {

    fn to_hex(&self) -> &'static str {      //has a static lifetime since I have hardcoded the hex reps

        match self {

        Color::Red => "#880808",
        Color::Green => "#7FFFD4",
        Color::Blue => "#00FFFF",
        }
    }

    fn is_red(&self) -> bool {

        if let Color::Red = self {

            true
        } else {

        false
        }
    }

}


#[derive(Debug)]
enum Animal {

    Dog(String),
    Cat(i32),
    Bird,

}


impl Animal {

    fn describe(&self) -> String {

        match self {

            Animal::Dog(name) => format!("Dog name: {}", name), 
            Animal::Cat(age) => "2".to_string(),
            Animal::Bird => "It's a bird!".to_string(),
      
        }
    }

    fn is_cat(&self) -> &'static str {

       if let Animal::Cat(_) = self {

           "Is a cat!"
       } else {

           "Not a cat!"
       }
    }

}


#[derive(Debug)]
enum Transport {

    Car(i32, String),
    Plane(i32),
    Boat(i32),
    Foot,
}

impl Transport {

    fn info(&self) -> String {

        match self {

            Transport::Car(speed, brand) => format!("Car of {} brand was traveling at {} km/hr", brand, speed),
            Transport::Plane(altitude) => format!("This plane is flying at {} feet", altitude),
            Transport::Boat(passengers) => format!("This boat has {} passengers", passengers),
            Transport::Foot => "You are walking".to_string(),
        }
    }

    fn is_plane(&self) -> &'static str {

        if let Transport::Plane(_) = self {

            "It's a Plane!"
        } else {

            "Not a plane!"
        }
 
    }

}



fn main() {
    let colour = Color::Red;
    println!("{:?}", colour.to_hex());


    let col2 = Color::Green;
    println!("{:?}", col2.is_red());


    let animal1 = Animal::Dog("Jimmy".to_string());
    println!("{}", animal1.describe());
    println!("{:?}", animal1.is_cat());


    let car = Transport::Car(80, "Toyota".to_string());
    println!("{}", car.info());
    println!("{:?}", car.is_plane());
}
