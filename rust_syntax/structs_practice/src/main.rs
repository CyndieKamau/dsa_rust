//Rust Profile for Checking User's Profile and Details
use std::io::{self, Write};  //imports std::io; and std::io::Write for io::stdout().flush()

#[derive(Debug)]
struct UserProfile {

    username: String,
    email: String,
    age: i32,

}

//implements function for struct to display info
impl UserProfile {

    fn display_info(username: String, email: String, age: i32) -> Self {

        UserProfile{username, email, age}
    }

}

//helper function to get user's input

fn user_input(prompt: &str) -> String {

    print!("{}", prompt);
    io::stdout().flush().unwrap();   //ensures the program's prompt is displayed


    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)
        .expect("Failed to read line");

    user_input.trim().to_string()   //removes newline chars, trim() returns &str so to_string() converts to String 
 
} 


fn main() {

    println!("Add user info below. Type 'exit' to quit");

    //store all User info in a vector
    let mut all_users: Vec<UserProfile>  = Vec::new();
 
    loop {

        let username = user_input("Type user's name: ");
        if username == "exit" {

            break;
        };

        let email = user_input("Type user's email address: ");
        if email == "exit" {

            break;
        };

        let age = user_input("Type user's age: ");
        if age == "exit" {

            break;
        };

        //convert age from String to i32
        let age: i32 = match age.parse() {

            Ok(num) => num,
            Err(_) => {

            println!("age must be a number");
            continue;

            },
        };

        //create a new UserProfile instance using user's input
        let user =  UserProfile::display_info(username, email, age);
        all_users.push(user);
        
    }
 
    //print all user info pushed to our vector
    println!("All Users");

    for user in all_users {

        println!("{:?}", user);
    }

}
