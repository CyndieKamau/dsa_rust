//Bubble Sort Algorithm
//Big O Notation of O(n^2)

//1. Start with the first element in the array
//2. If the first item is greater than the second, swap them
//3. Move to the next item (3rd item), repeat step 2 till end of array
//N.B. You'll notice automatically the largest item in array will be sorted "bubbled up" at the end of the array
//4. Repeat the same steps, excluding the last element as its already sorted
//5. Continue iterating while excluding last item, till the array is sorted


fn bubble_sort(array: &mut[i32]) {

    let mut arr = array.len();    //getting the length determines no of iteration times

    let mut swapped;    //we'll use this to check if items have swapped or not

    loop {

        swapped = false;   //no initial swapping has occured

        for i in 1..n {

            if arr[i-1] > arr[i] {

                arr.swap(i - 1, i);
                swapped = true; 

            }

        }
  
    }


}  




fn main() {
    println!("Hello, world!");
}
