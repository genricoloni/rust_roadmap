use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for &num in &nums {
        *map.entry(num).or_insert(0) += 1;
    }

    let mut freq: Vec<_> = map.into_iter().collect();
    freq.sort_by(|a, b| b.1.cmp(&a.1));

    freq.iter().take(k as usize).map(|&(num, _)| num).collect()
}