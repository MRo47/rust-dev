use std::io;

fn main() {
    // int128 can use _ for readability
    let x: i128 = 123_456_789;
    println!("x = {x}");

    let _x = 3.2; // default f64

    let quotient = 3.2 / 1.5;
    println!("3.2 / 1.5 = {quotient}");

    let truncated = -5 / 3;
    println!("-5 / 3 = {truncated}");

    let remainder = 45 % 6;
    println!("45 % 6 = {remainder}");

    let emoji = "🐈"; // supports unicode
    println!("cat = {emoji}");

    // tuples
    let tup: (i32, f64, bool) = (500, 4.5, false);

    // destructuring
    let (x, y, z) = tup;
    println!("tuple = ({x}, {y}, {z})");

    // indexing
    // unpack before using
    let tup_0 = tup.0;
    let tup_1 = tup.1;
    let tup_2 = tup.2;
    println!("tuple = ({tup_0}, {tup_1}, {tup_2})");

    // unit: return value same as type
    let _z: () = ();

    // arrays
    let _a = [1, 2, 3, 4];

    // explicitly typed array
    let _a: [i64; 5] = [1, 2, 3, 4, 5];

    // array of 5 elements with value 4
    let _a = [4; 5];

    let _first_a = _a[0];

    let mut index = String::new();

    println!("Enter array index");

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index is not a number");

    let element = _a[index];

    println!("Array a at index: {index} is {element}");
}
