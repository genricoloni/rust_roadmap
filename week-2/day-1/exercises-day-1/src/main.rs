use std::io;

fn main() {
    // Prendi un input di stringa dall'utente
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim().to_string(); // Rimuovi eventuali spazi bianchi iniziali e finali e converti in String

    let result = string_length(s.clone());
    println!("The length of the string is {}", result);

    let (first, rest) = first_word(s.clone());
    println!("First word: {}, Rest: {}", first, rest);

    let result = unique_characters(s);
    println!("The unique characters are {}", result);
}

// 1. [Easy] Write a function that takes a `String` and returns its length.
fn string_length(s: String) -> usize {
    s.len()
}

// 2. [Medium] Write a function that takes a `String` and returns the first word in it. The function should return a tuple with the first word and the rest of the string.
fn first_word(s: String) -> (String, String) {
    let mut first = String::new();
    let mut rest = String::new();
    let mut found_space = false;

    for c in s.chars() {
        if c == ' ' && !found_space {
            found_space = true;
        } else if found_space {
            rest.push(c);
        } else {
            first.push(c);
        }
    }

    (first, rest)
}

// 3. [Hard] Write a function that takes a `String` and returns a new `String` containing only the unique characters from the original string, preserving their order of first appearance.
fn unique_characters(s: String) -> String {
    let mut unique_chars = String::new();
    let mut seen = std::collections::HashSet::new();

    for c in s.chars() {
        if c == ' ' {
            continue;
        }
        if !seen.contains(&c) {
            unique_chars.push(c);
            seen.insert(c);
        }
    }

    unique_chars
}