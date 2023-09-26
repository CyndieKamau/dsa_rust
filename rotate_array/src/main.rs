
//write a function `rotate_array`
//It takes in a vector of arrays, and an integer k as arguments
//It rotates the values of the vector by `k` steps
//eg [1,3,2,4] by 2 steps is [2,6,4,8]



fn rotate_array(mut arr: Vec<i32>, k: i32) -> Vec<i32> {

    for x in 0..arr.len() {

        arr[x] *= k;    //multply each element in array by k

    }
    arr   //return the vector
} 


fn main() {

    let z = rotate_array(vec![2,3,7,1,5], 3);

    println!("{:?}", z);    

}
