// arithmetic
fn inter_product(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}

// type inference
fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(x: i8) {
    println!("i8: {x}");
}

fn fib(n: i32) -> i32 {
    assert!(n > -1);
    if n < 2 {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn main() {
    println!("Hello, world!");

    // arithmetic
    println!("inter product: {}", inter_product(3, 4, 5));

    // type inference
    let x = 32;
    let y = 10;

    takes_i8(x);
    takes_u32(y);

    // fibonacci recusrsion
    let n: i32 = 10;
    println!("fib({n}) = {}", fib(n));
}
