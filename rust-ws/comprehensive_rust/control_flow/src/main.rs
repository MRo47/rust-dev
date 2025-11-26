fn main() {
    // scopes
    let z = 13;
    let x = {
        let y = 10;
        dbg!(y);
        z - y // takes this value but adding ; will make this type and value ()
    };
    dbg!(x);
    // dbg!(y); // cannot find y in this scope

    // if condition
    let x = 10;
    if x == 0 {
        println!("zero");
    } else if x < 100 {
        println!("biggish");
    } else {
        println!("yuge");
    }

    // conditional assign
    let size = if x < 20 { "small" } else { "large" };
    println!("size: {size}");

    // match expressions
    let val = 1;
    match val {
        1 => println!("one"),
        10 => println!("ten"),
        100 => println!("hundread"),
        _ => {
            println!("something else");
        }
    }

    let flag = true;
    let val = match flag {
        true => 1,
        false => 0,
    };
    println!("flag: {flag}, value: {val}");

    // loops
    let mut x = 200;
    while x > 10 {
        x /= 2;
    }
    dbg!(x);

    for x in 1..5 {
        dbg!(x);
    }

    // inclusive range
    for x in 1..=5 {
        dbg!(x);
    }

    for elem in [1, 2, 3, 4, 5] {
        dbg!(elem);
    }

    let mut i = 0;
    // infinite loop: while true
    loop {
        i += 1;
        if i % 2 == 0 {
            dbg!(i);
            continue;
        }
        if i >= 10 {
            dbg!(i);
            break;
        }
    }

    // loop and scope labels
    // both continue and break can use labels to break out of nested loops
    let s = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let target_val = 6;
    let mut visited = 0;
    'outer: for i in 0..3 {
        for j in 0..3 {
            visited += 1;
            if s[i][j] == 2 {
                continue 'outer;
            }

            if s[i][j] == target_val {
                break 'outer;
            }
        }
    }
    dbg!(visited);

    // block label
    'label: {
        break 'label;
        println!("will never print");
    }

    dbg!(gcd(143, 52));

    // println!(format, ..) prints a line to standard output, applying formatting described in std::fmt.
    // format!(format, ..) works just like println! but returns the result as a string.
    // dbg!(expression) logs the value of the expression and returns it.
    // todo!() marks a bit of code as not-yet-implemented. If executed, it will panic.
    // assert! and related macros can be used to add assertions to your code. These are used heavily in writing tests.
    // unreachable! is used to mark a branch of control flow that should never be hit.
    // eprintln! allows you to print to stderr.

    // collatz exercise
    let n = 11;
    println!("collatz length of {n} = {}", collatz_length(n));
}

fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 {
        gcd(b, a % b) // this is a return value, no ;
    } else {
        a // statement or block without ; at the end becomes the return value for the function
    }
}

// book soln
// fn collatz_length(mut n: i32) -> u32 {
//     let mut len = 1;
//     while n > 1 {
//         n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
//         len += 1;
//     }
//     len
// }

// my soln
fn collatz_length(n: i32) -> i32 {
    assert!(n > 0);

    if n == 1 {
        return 1;
    }

    fn collatz(i: i32) -> i32 {
        if i == 1 {
            i // terminate condition
        } else if i % 2 == 0 {
            i / 2
        } else {
            3 * i + 1
        }
    }

    let mut seq_len = 1; // will have atleast length 1 since n is not 1
    let mut next = collatz(n);
    while next > 1 {
        next = collatz(next);
        seq_len += 1;
    }

    seq_len + 1 // add the final length of 1
}
