use std::io;

fn main() {
    let mut x = String::new();

    println!("enter a number");

    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read input line");

    let x = x.trim().parse().expect("input is not a number");

    let fib_x = fibonacci(x);
    println!("fibonacii of {x} = {fib_x}");
}

fn fibonacci(x: i64) -> i64 {
    if x == 0 {
        return 0;
    } else if x == 1 {
        return 1;
    }

    x + fibonacci(x - 1)
}
