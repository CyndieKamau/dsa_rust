//Here we will look at Results, and Options

#[derive(Debug)]
pub enum Res<T, E> {

    Thing(T),
    Error(E),

}


pub fn divide(a: i32, b: i32) -> Res<i32, String> {

    if b == 0 {
  
        return Res::Error("Cannot divide by 0".to_string());
    }
    
    Res::Thing(a/b)

}



fn main() {

    let a = divide(4, 5);
    let b = divide(10, 0);
    println!(" a = {:?}, b = {:?}", a, b);
}
