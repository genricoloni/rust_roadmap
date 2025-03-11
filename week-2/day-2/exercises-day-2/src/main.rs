use std::collections::HashSet;

fn main() {
    let s = String::from("hello world this is Rust");
    let result = reverse_words(&s);
    println!("Reversed words: {}", result); 

    let mut nums = vec![1, 2, 2, 3, 4, 4, 5];
    remove_duplicates(&mut nums);
    println!("After removing duplicates: {:?}", nums); // Output: [1, 2, 3, 4, 5]

}

fn remove_duplicates(nums: &mut Vec<i32>) {
    let mut seen: HashSet<i32> = HashSet::new();

    let mut i = 0;
    while i < nums.len() {
        if seen.contains(&nums[i]) {
            nums.remove(i);
        } else {
            seen.insert(nums[i]);
            i += 1;
        }
    }

}


// 1. - [Medium] Write a function that receives a reference to a `String` and returns a new `String` with words in reverse order. For example, if the input is `"hello world"`, the output should be `"world hello"`.
fn reverse_words(s: &String) -> String {
    let mut words = s.split_whitespace();
    let mut reversed = String::new();
    let mut first = true;
    for word in words {
        if first {
            reversed = word.to_string();
            first = false;
        } else {
            reversed = format!("{} {}", word, reversed);
        }
    }
    reversed
}

