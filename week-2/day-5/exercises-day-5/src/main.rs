
fn main() {
    // 1: wovel or consonant
    let chars = ['a', 'z', '4', 'o'];
    for &c in &chars {
        println!("'{}' => {}", c, classify_char(c));
    }

    // 2: Merge two sorted arrays
    let slice1 = [1, 3, 5, 7];
    let slice2 = [2, 6, 6, 10, 12];
    let merged = merge_sorted(&slice1, &slice2);
    println!("Merged: {:?}", merged); // [1, 2, 3, 5, 6, 6, 7, 10, 12]

    // 3: Two sum
    let nums = [2, 7, 11, 15];
    let target = 19;
    match two_sum(&nums, target) {
        Some((i, j)) => println!("Indices: ({}, {})", i, j),
        None => println!("No two sum found"),
    }
    // Output: Indices: (0, 1)

    // 4: Mini expression evaluator
    use Expr::*;
    let expr = Add(
        Box::new(Mul(Box::new(Number(2)), Box::new(Number(3)))),
        Box::new(Number(5)),
    );
    
    match evaluate(&expr) {
        11 => println!("Correct!"),
        _ => println!("Incorrect!"),
    }

    let expr = Mul(
        Box::new(Add(Box::new(Number(2)), Box::new(Number(3)))),
        Box::new(Number(5)),
    );

    match evaluate(&expr) {
        25 => println!("Correct!"),
        _ => println!("Incorrect!"),
    }

    // 5: Rotate slices in-place
    let mut arr = [1, 2, 3, 4, 5];
    rotate_in_place(&mut arr, 2);
    println!("{:?}", arr); // [4, 5, 1, 2, 3]
}


// 1: wovel or consonant
fn classify_char(c: char) -> String {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => "vowel".to_string(),
        x if x.is_alphabetic() => "consonant".to_string(),
        _ => "neither".to_string(),
    }
}

// 2: Merge two sorted arrays
fn merge_sorted(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);

    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            result.push(a[i]);
            i += 1;
        } else {
            result.push(b[j]);
            j += 1;
        }
    }

    // Push remaining elements
    result.extend_from_slice(&a[i..]);
    result.extend_from_slice(&b[j..]);
    result
}

// 3: Two sum
fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut map = std::collections::HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        if let Some(&j) = map.get(&(target - num)) {
            return Some((j, i));
        }
        map.insert(num, i);
    }
    None
}

// 4: Mini expression evaluator
enum Expr {
    Number(i32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

fn evaluate(expr: &Expr) -> i32 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Add(left, right) => evaluate(left) + evaluate(right),
        Expr::Mul(left, right) => evaluate(left) * evaluate(right),
    }
}


// 5: Rotate slices in-place
fn rotate_in_place(data: &mut [i32], k: usize) {
    let len = data.len();
    if len == 0 { return; }

    let k = k % len;
    // Reverse the entire slice
    data.reverse();
    // Reverse the first k elements
    data[..k].reverse();
    // Reverse the remaining len-k elements
    data[k..].reverse();
}
