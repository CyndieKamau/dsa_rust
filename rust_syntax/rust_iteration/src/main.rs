// This program says hello to anyone whose name starts with 'W'
// The rest of the names are just printed out.

fn say_hello_simple(name: &str) {

    if let Some(next_ch) = name.chars().next() {

        if next_ch == 'W' {

            println!("Hello, {}", name);

        } else {

            println!("{}", name);
        }

    }

}


fn say_hello_complex(name: &str) {

    let mut characters = name.chars().peekable();

    if let Some(&ch) = characters.peek() {

        match ch {

            'W' => {

                println!("Hello, {}", name);
            },

            _ => {

                println!("{}", name);
            }
        }
    }

}


fn main() {
    say_hello_simple("Winfred");
    say_hello_simple("Mikey");


    say_hello_complex("Wangechi");
    say_hello_complex("Alice");
}
