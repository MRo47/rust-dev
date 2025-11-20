use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing game");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("guess a number");

        // mutable type, default types are immutable
        // type is guessed automatically here
        let mut guess = String::new();

        // handle error result
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadowing to reinit the variable
        // match expression to handle result and error
        // type specification unsigned int 32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // cmp 2 numbers will produce results of 3 types
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };

        println!("you guessed {}", guess);
    }
}
