use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let mut s_map = HashMap::new();
    let mut t_map = HashMap::new();

    for c in s.chars() {
        *s_map.entry(c).or_insert(0) += 1;
    }

    for c in t.chars() {
        *t_map.entry(c).or_insert(0) += 1;
    }

    s_map == t_map
}