use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number between 1 and 100");
    
    loop {
        println!("Enter your guess:");

        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        //println!("Secret number: {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too large!\n"),
            Ordering::Equal => { println!("You win!"); break; },
        }
    }
}
