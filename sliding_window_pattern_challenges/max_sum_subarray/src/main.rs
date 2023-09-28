//Sliding window design pattern challenge
//find the maximum Sum Subarray of Size `k`
//use slices to maintain window and keep track of the sum
//1. You maintain a window of size `k`, and compute elements in this window
//2. The window with highest sum gets recorded
//3.As you slide through the window, add the elements in it, subtract element sliding out the window
//4. You'll take a reference of the array, and int `k` as arguments


fn max_sum_subarray(arr: &[i32], k: usize) -> Option<usize> {

    let n = arr.len();

    //if k is larger than length of array return None; 
    //That's why I used `Option` enum` for graceful error handling 

    if n < k {

        return None;

    }

    //Compute the sum of the first window
    let mut window_sum: i32 = arr[..k].iter().sum();
    let mut max_sum = window_sum;

    //slide the window through the array to the right
    for i in k..n {

        //update window sum by subtracting first element in the window, and adding a new element
        window_sum += arr[i] - arr[i - k];
        
        //Update the max_sum
        max_sum = max_sum.max(window_sum);
    }

    //convert to usize from an i32
    Some(max_sum.try_into().unwrap())



}

fn main() {
    let arr = [2,3,1,4,3,2,5,1,6];
    let k = 3;

    match max_sum_subarray(&arr, k) {

        Some(max_sum) => {

            println!("The max sum in the subarray of size {} is {}", k, max_sum) 
        },

        None => {

            println!("Size {} is bigger than the array", k)
        },
    }
}
