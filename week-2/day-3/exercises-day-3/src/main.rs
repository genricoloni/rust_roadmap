fn main() {
    let text = "hello world\nthis is Rust\n\n   \n";
    let lines = split_lines(text);
    for line in lines {
        println!("{}", line);
    }

    let arr = [10, 2, 3, 14, 5, 6, 7, 8, 9];
    let size = 3;
    let max_slices = max_subarray(&arr, size);
    println!("Max slices with size {}: {:?}", size, max_slices);
}

//Write a function that split a multi-line string into a vector of slices, where each slice is a line of the string, ignoring the newline character and empty lines.
fn split_lines(s: &str) -> Vec<&str> {
    let mut lines = Vec::new();
    let mut start = 0;
    let mut end = 0;
    for chr in s.chars() {
        if chr == '\n' {
            if start != end && !s[start..end].trim().is_empty() {
                lines.push(&s[start..end]);
            }
            start = end + 1;
        }
        end += 1;
    }
    // Aggiungi l'ultima linea se non è vuota
    if start != end && !s[start..end].trim().is_empty() {
        lines.push(&s[start..end]);
    }
    lines

    // another, faster and Rustier way to do it is:
    // s.lines().filter(|line| !line.is_empty()).collect()
}

// Write a function that takes an array and a size, and returns the sub-slices of the array with the given size whose sum is the maximum possible.
fn max_subarray(arr: &[i32], size: usize) -> Vec<&[i32]> {
    let mut max_sum = 0;
    let mut max_slices = Vec::new();
    for i in 0..arr.len() - size + 1 {
        let slice = &arr[i..i + size];
        let sum: i32 = slice.iter().sum();
        if sum > max_sum {
            max_sum = sum;
            max_slices.clear();
            max_slices.push(slice);
        } else if sum == max_sum {
            max_slices.push(slice);
        }
    }
    max_slices
    
}
