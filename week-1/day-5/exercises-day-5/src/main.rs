struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut new_array: Vec<i32> = Vec::with_capacity(nums.len() * 2);
        //let mut new_array: Vec<i32> = Vec::new();
        

        new_array.extend_from_slice(&nums);
        new_array.extend_from_slice(&nums);

        new_array
    }

    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut res = String::new();
        let mut iter1 = word1.chars();
        let mut iter2 = word2.chars();

        loop {
            match(iter1.next(), iter2.next()){
                (Some(c1), Some(c2)) => {
                    res.push(c1);
                    res.push(c2);
                }
                (Some(c1), None) => res.push(c1),
                (None, Some(c2)) => res.push(c2),
                (None, None) => break,
            }
        }
        res
    }

    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut count = HashMap::new();
        let mut duplicate = 0;
        let mut missing = 0;

        for &num in &nums {
            let count = count.entry(num).or_insert(0);
            *count += 1; 
        }

        for i in 1..nums.len() as i32 + 1 {
            match count.get(&i) {
                Some(&count) if count == 2 => duplicate = i,
                None => missing = i,
                _ => ()
                
            }
        }
        vec![duplicate, missing]
    }
}

fn main() {
    let nums = vec![1, 2, 3];
    let result: Vec<i32> = Solution::get_concatenation(nums);
    println!("{:?}", result); // Output: [1, 2, 3, 1, 2, 3]

    let word1 = String::from("abc");
    let word2 = String::from("pqr");
    let result = Solution::merge_alternately(word1, word2);
    println!("{}", result); // Output: "apbqcr"

    let nums = vec![1, 1];
    let result = Solution::find_error_nums(nums);
    println!("{:?}", result); 

}

//concatenation of arrays