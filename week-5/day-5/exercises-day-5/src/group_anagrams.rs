use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();

    for s in strs {
        let mut chars = s.chars().collect::<Vec<_>>();
        chars.sort();
        let key = chars.into_iter().collect::<String>();
        map.entry(key).or_insert(vec![]).push(s);
    }

    map.into_iter().map(|(_, v)| v).collect()
}