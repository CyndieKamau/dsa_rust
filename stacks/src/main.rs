

fn main() {
    let mut a = vec!(1,2,3,4,5);
    println!("List: {:?}", a);

    let b = &mut a;
    b.push(6);

    for x in b {

        println!("{:?}", x);

    }
    //println!("List 2: {:?}", b);
}
