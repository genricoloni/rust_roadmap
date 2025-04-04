use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut map = HashMap::new();
    let mut left = 0;
    let mut max_length = 0;

    for (right, c) in s.chars().enumerate() {
        if let Some(&index) = map.get(&c) {
            left = left.max(index + 1);
        }
        map.insert(c, right);
        max_length = max_length.max(right - left + 1);
    }

    max_length as i32
}