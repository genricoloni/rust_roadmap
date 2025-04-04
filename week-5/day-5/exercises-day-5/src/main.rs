mod two_sum;
mod is_anagram;
mod group_anagrams;
mod top_k_frequent;
mod length_of_longest_substring;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        two_sum::two_sum(nums, target)
    }

    pub fn is_anagram(s: String, t: String) -> bool {
        is_anagram::is_anagram(s, t)
    }

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        group_anagrams::group_anagrams(strs)
    }

    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        top_k_frequent::top_k_frequent(nums, k)
    }

    pub fn length_of_longest_substring(s: String) -> i32 {
        length_of_longest_substring::length_of_longest_substring(s)
    }
}

fn main() {
    // --- Test per two_sum ---
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert!(Solution::two_sum(nums, target) == vec![0, 1]);

    let nums = vec![3, 2, 4];
    let target = 6;
    assert!(Solution::two_sum(nums, target) == vec![1, 2]);

    let nums = vec![3, 3];
    let target = 6;
    assert!(Solution::two_sum(nums, target) == vec![0, 1]);

    let nums = vec![1, 2, 3, 4, 5];
    let target = 10;
    assert!(Solution::two_sum(nums, target) == vec![]); // Correzione qui

    let nums = vec![-1, -3, 5, 7];
    let target = 4;
    assert!(Solution::two_sum(nums, target) == vec![0, 2]);

    let nums = vec![];
    let target = 5;
    assert!(Solution::two_sum(nums, target) == vec![]);

    let nums = vec![5];
    let target = 10;
    assert!(Solution::two_sum(nums, target) == vec![]);
    // --- Test per is_anagram ---
    let s = String::from("anagram");
    let t = String::from("nagaram");
    assert!(Solution::is_anagram(s, t) == true);

    let s = String::from("rat");
    let t = String::from("car");
    assert!(Solution::is_anagram(s, t) == false);

    let s = String::from("a");
    let t = String::from("ab");
    assert!(Solution::is_anagram(s, t) == false);

    let s = String::from("aabb");
    let t = String::from("bbaa");
    assert!(Solution::is_anagram(s, t) == true);

    let s = String::from("");
    let t = String::from("");
    assert!(Solution::is_anagram(s, t) == true);

    let s = String::from("abc");
    let t = String::from("abcd");
    assert!(Solution::is_anagram(s, t) == false);

    let s = String::from("listen");
    let t = String::from("silent");
    assert!(Solution::is_anagram(s, t) == true);

    // --- Test per group_anagrams ---
    let strs = vec![String::from("eat"), String::from("tea"), String::from("tan"), String::from("ate"), String::from("nat"), String::from("bat")];
    let result = Solution::group_anagrams(strs);
    let expected = vec![
        vec![String::from("eat"), String::from("tea"), String::from("ate")],
        vec![String::from("tan"), String::from("nat")],
        vec![String::from("bat")],
    ];
    assert!(result.len() == expected.len());
    for group in result {
        assert!(expected.iter().any(|g| {
            g.len() == group.len() && g.iter().all(|item| group.contains(item))
        }));
    }

    let strs = vec![String::from("")];
    let result = Solution::group_anagrams(strs);
    let expected = vec![vec![String::from("")]];
    assert!(result == expected);

    let strs = vec![String::from("a")];
    let result = Solution::group_anagrams(strs);
    let expected = vec![vec![String::from("a")]];
    assert!(result == expected);

    let strs = vec![String::from("abc"), String::from("def"), String::from("ghi")];
    let result = Solution::group_anagrams(strs);
    assert!(result.len() == 3);
    assert!(result.iter().any(|g| g == &vec![String::from("abc")]));
    assert!(result.iter().any(|g| g == &vec![String::from("def")]));
    assert!(result.iter().any(|g| g == &vec![String::from("ghi")]));

    let strs = vec![String::from("aabb"), String::from("bbaa"), String::from("abcd"), String::from("bacd"), String::from("ddcc")];
    let result = Solution::group_anagrams(strs);
    assert!(result.len() == 3);
    assert!(result.iter().any(|g| {
        g.len() == 2 && g.contains(&String::from("aabb")) && g.contains(&String::from("bbaa"))
    }));
    assert!(result.iter().any(|g| {
        g.len() == 2 && g.contains(&String::from("abcd")) && g.contains(&String::from("bacd"))
    }));
    assert!(result.iter().any(|g| g == &vec![String::from("ddcc")]));

    let strs = vec![];
    let result = Solution::group_anagrams(strs);
    assert!(result.is_empty());

    // --- Test per top_k_frequent ---
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    let result = Solution::top_k_frequent(nums, k);
    let expected = vec![1, 2];
    assert!(result.len() == expected.len());
    assert!(result.contains(&1));
    assert!(result.contains(&2));

    let nums = vec![1];
    let k = 1;
    let result = Solution::top_k_frequent(nums, k);
    let expected = vec![1];
    assert!(result.len() == expected.len());
    assert!(result.contains(&1));

    let nums = vec![1, 2];
    let k = 2;
    let result = Solution::top_k_frequent(nums, k);
    assert!(result.len() == 2);
    assert!(result.contains(&1));
    assert!(result.contains(&2));

    let nums = vec![1, 2, 2, 3, 3, 3];
    let k = 1;
    let result = Solution::top_k_frequent(nums, k);
    let expected = vec![3];
    assert!(result == expected);

    let nums = vec![1, 2, 2, 3, 3, 3];
    let k = 3;
    let result = Solution::top_k_frequent(nums, k);
    assert!(result.len() == 3);
    assert!(result.contains(&1));
    assert!(result.contains(&2));
    assert!(result.contains(&3));

    let nums = vec![];
    let k = 1;
    let result = Solution::top_k_frequent(nums, k);
    assert!(result.is_empty());

    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 0;
    let result = Solution::top_k_frequent(nums, k);
    assert!(result.is_empty());

    let nums = vec![-1, -1, 2, 3];
    let k = 2;
    let result = Solution::top_k_frequent(nums, k);
    assert!(result.len() == 2);
    assert!(result.contains(&-1));
    assert!(result.contains(&2) || result.contains(&3)); // Order might vary

    // --- Test per length_of_longest_substring ---
    let s = String::from("abcabcbb");
    let result = Solution::length_of_longest_substring(s);
    let expected = 3;
    assert!(result == expected);

    let s = String::from("bbbbb");
    let result = Solution::length_of_longest_substring(s);
    let expected = 1;
    assert!(result == expected);

    let s = String::from("pwwkew");
    let result = Solution::length_of_longest_substring(s);
    let expected = 3;
    assert!(result == expected);

    let s = String::from("");
    let result = Solution::length_of_longest_substring(s);
    let expected = 0;
    assert!(result == expected);

    let s = String::from(" ");
    let result = Solution::length_of_longest_substring(s);
    let expected = 1;
    assert!(result == expected);

    let s = String::from("au");
    let result = Solution::length_of_longest_substring(s);
    let expected = 2;
    assert!(result == expected);

    let s = String::from("dvdf");
    let result = Solution::length_of_longest_substring(s);
    let expected = 3;
    assert!(result == expected);

    let s = String::from("abacaba");
    let result = Solution::length_of_longest_substring(s);
    let expected = 3;
    assert!(result == expected);
}