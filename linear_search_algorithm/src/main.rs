//Linear Search Algorithm is the simplest search algorithm around.
//Basically works concurrently with Arrays `[]` the simplest data structure.
//Indexing in arrays is a form of linear search algorithm.
//Has a big O notation of O(n)

//1. Define a function `linear_search` -> 2 parameters; The array(referenced) and target search value
//2. Function uses Option<usize>,returns Some(index) if value is found, None if its absent from the array
//3. For loop to iterate over all the items in the array(rem array is borrowed), while comparing each item with target value
//4. If match, return Some(index). If not return None. 
//5. Exit loop



fn linear_search(array: &[i32], target: i32) -> Option<usize> {

    for(index, &item) in array.iter().enumerate() {

        if item == target {

            return Some(index);
        } 
    }
    None
}


fn main() {
    let x: [i32; 7] = [7,5,3,6,1,4,2];

    let target = 4;

    let y = linear_search(&x, target);
   
    println!("{:?}", y);    //for lazily printing out the output in console


    let target = 6;

    match linear_search(&x, target) {

        Some(index) => println!("Target {} found in index {}", target, index),
 
        None => println!("Target {} not found", target),
  
    }
}
