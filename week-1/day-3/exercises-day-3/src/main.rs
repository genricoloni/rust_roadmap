

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number: i32 = input.trim().parse().expect("Please type a number");
    


    //1: is_even
    let result: bool = is_even(number);
    println!("Is the number even? {}", result);

    let result: i32 = recursive_fibonacci(number);
    println!("Fibonacci result:{}", result);

    let result: i64 = recursive_factorial(number);
    println!("{number}! = {result}");

    let result: i64 = factorial(number);
    println!("{number}! = {result}");

    'exercise_4: loop {
        println!("Menu:");
        println!("1. Calculate the nth Fibonacci number");
        println!("2. Calculate the factorial of a number");
        println!("3. Calculate both Fibonacci and Factorial");
        println!("4. Exit");

        println!("Enter your choice:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: i32 = choice.trim().parse().expect("Please type a number");

        if choice == 1 {
            println!("Enter the value of n:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let number: i32 = input.trim().parse().expect("Please type a number");

            let fib_result = fibonacci(number);
            println!("Fibonacci result: {}", fib_result);

        } else if choice == 2 {
            println!("Enter the number:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let number: i32 = input.trim().parse().expect("Please type a number");

            let fact_result = factorial(number);
            println!("Factorial result: {}", fact_result);
        } else if choice == 3 {

            println!("Enter the number:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let number: i32 = input.trim().parse().expect("Please type a number");

            let result = fib_fac(number);
            println!("Fibonacci result: {}", result.0);
            println!("Factorial result: {}", result.1);

        } else if choice == 4{
            println!("Exiting...");
            break 'exercise_4;

        } else {
            println!("Invalid choice, please try again.");

        }
    }

}

//1: is_even
fn is_even(number: i32) -> bool {
    if number % 2 == 0{
        return true
    }
    false
    
}

//2: recursive fibonaccio
fn recursive_fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

//2: iterative fibonacci
fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    let mut a = 0;
    let mut b = 1;
    let mut fib = 0;
    for _ in 2..=n {
        fib = a + b;
        a = b;
        b = fib;
    }
    fib
}

//3: recuriseve factorial
fn recursive_factorial(number : i32) -> i64{
    if number <= 1 {
        return 1
    }
    number as i64 * recursive_factorial(number - 1)
}

//3: iterative factorial
fn factorial(mut number : i32) -> i64{
    let mut res: i64 = 1;

    while number > 1 {
        res *= number as i64;
        number -= 1;
    }
    res
}



//4: Fiboncaii and Factorial
fn fib_fac(number : i32) -> (i32, i64) {
    let res_fib = fibonacci(number);
    let res_fac = factorial(number);

    return (res_fib, res_fac)
}