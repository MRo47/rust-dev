fn main() {
    println!("Control Flow");

    let number = 3;

    if number < 5 {
        println!("number < 5");
    } else {
        println!("number >= 5");
    }

    // conditions expect bool only, no type conversion
    // if number {
    //     println!("Error")
    // }

    if number != 0 {
        println!("number not zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number divisible by 4");
    } else if number % 3 == 0 {
        println!("number divisible by 3");
    } else if number % 2 == 0 {
        println!("number divisible by 2");
    } else {
        println!("number not divisible by 2,3 nor 4");
    }

    // conditional assignment
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("number = {number}");

    // error since types are incompatible
    // let number = if condition { 5 } else { "six" };

    // loops

    // return values from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("loop result = {result}");

    // loop labels to disambiguate between loops

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // from innner loop
            }
            if count == 2 {
                break 'counting_up; // from labelled loop
            }

            remaining -= 1;
        }

        count += 1;
    }
    println!("end count = {count}");

    // conditional while loop

    let mut number = 4;

    while number > 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF");

    // loop through a collection

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("value = {element}");
    }

    // ranged loop
    // rev will reverse the range
    for element in (1..4).rev() {
        println!("value = {element}");
    }
}
