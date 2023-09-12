//Binary Search Algorithm is a classic search algorithm.
//Used in sorted data -> sorted vectors, arrays
//Divides sorted data in half by every iteration till desired value is found.
//Has Big O Notation of O(logn)
//1. Create a function `binary_search` -> has &array, target value as parameters, returns `Option<usize>`
//2. Initialize `low`, `high` pointers ; low -> 0, high -> array.length -1
//3. While loop -> while low =< high;
//3.1 get middle index of items in array  -> mid = (low + high) /2
//3.2 Compare item at mid index to target value
//3.3 If target value == mid index item return `Some(mid index)`   we've found the target  
//3.3 If target value > mid index item;
//3.4 low == mid value + 1  (target value is in right half of array)
//3.5 Else if target value < mid index item;
//3.6 high == mid value - 1  (target value is in left half of array)
//3.7 Continue iterating till target is found
//3.8 If low > high, return `None`; target not in array


use std::cmp::Ordering;


fn binary_search(array: &mut [i32], target: i32) -> Option<usize> {

   let mut low = 0;
   let mut high = array.len() -1;

   while low <= high {
       
       let mid_index = (low + high) /2;
       
       match target.cmp(&mut array[mid_index]) {

           Ordering::Equal => return Some(mid_index),

           Ordering::Greater => {

               low = mid_index + 1; 
           },

           Ordering::Less => {

               //prevent underflow
               if mid_index == 0 {

                   break;
               }

               high = mid_index - 1;
           },
       } 
   }

   None    //if low > high, no value in array return None

}



fn main() {
    let mut array: [i32; 10] =  [3,2,6,7,5,8,3,9,4,1];   //to sort the array it needs to be mutable

    array.sort();   //binary search algorithm works only on sorted data

    let target = 10;

    match binary_search(&mut array, target) {

    Some(index) => println!("Target {} found in index {}", target, index),
    None => println!("Target {} not found in array", target),

    }

}
