//Create a function `max_unique_chars` with two parameters, reference to string and `k`
//`k` should be an integer
//Handle edge cases first; if `k` is 0, or string is empty, return None
//Initialize a start pointer and end pointer
//Contents between the sliding window are stored in a hashmap `char_freq` as it stores count of each character
//initialize a variable `max_length` to store max count of char sequence
//Expand over sliding window by iterating using the end pointer
//For each char in end pointer, update hashmap and check count
//After adding a new char in the window, check if `k` exceeds the chars
//If it exceeds, shrink the window till chars are almost equal to `k`
//After each step, calculate window's length, if its higher than `max_length`, update 


use std::collections::HashMap;


fn max_unique_chars(string: &str, k: usize) -> Option<String> {

    if k == 0 || string.is_empty() {

        return None;
 
    }

    let mut char_freq = HashMap::new();
    let mut start_point = 0;
    let mut max_length = String::new();

    for (end_point, char) in string.char_indices() {

        //updating and inserting character frequencies in the sliding window
        *char_freq.entry(char).or_insert(0) += 1;

        //contracting the window
        while char_freq.len() > k {

            let start_char = string.chars().nth(start_point).unwrap();
            *char_freq.get_mut(&start_char).unwrap() -= 1;

            if char_freq[&start_char] == 0 {

                char_freq.remove(&start_char);
            }

            start_point += 1
            
        }

        if end_point - start_point + 1 > max_length.len() {

            max_length = string[start_point..=end_point].to_string();
        }

    }

    Some(max_length)


}


fn main() {
    let string = "asggeehryddwe";
    let k = 3;

    match max_unique_chars(&string, k) {

        Some(max_length) => {

            println!("The max length of unique chars of size {} is {}", k, max_length) 
        },

        None => {

            println!("{} is zero, or string is empty", k)
        },
    }
}
