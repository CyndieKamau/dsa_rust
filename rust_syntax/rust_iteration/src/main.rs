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


fn main() {
    say_hello_simple("Winfred");
    say_hello_simple("Mikey");
}
