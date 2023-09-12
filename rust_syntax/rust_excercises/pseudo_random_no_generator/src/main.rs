//Write a program that generates 10 pseudo_random f32 numbers between 100 and 400 




fn main() {
    let x = vec![5,7,8,3,6];

    let second_element: &u32 = &x[1];

    println!("The second value in vector is {}", second_element);

    let third_element_get: Option<&u32> = x.get(2);

    match third_element_get { 

    Some(third_element_get) => println!("The third value in vector is {}", third_element_get),

    None => println!("No third value here"),
    } 

}
