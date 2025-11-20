fn main() {
    println!("Functions");

    print_time(5, 'h');

    let x = five();

    println!("x = {x}");

    let y = plus_one(x);

    println!("y = {y}");
}

fn print_time(hour: i32, unit: char) {
    println!("Time: {hour}{unit}");
}

// function with a declared return type
// here return value is synonymous with the final expression (note it is not a statement)
// return early using `return` keyword, no need for final expression
fn five() -> i32 {
    5
}

// note final line is an expression which returns the evaluated value
// note the missing semicolon, which would make it a statement (no return values)
fn plus_one(x: i32) -> i32 {
    x + 1
}
