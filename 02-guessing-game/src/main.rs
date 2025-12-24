use rand::Rng;
use std::io;
use std::io::Write;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    let secret_number: u32 = rand::rng().random_range(1..=100);
    println!("The secret number is: {}", secret_number);

    let mut count: u8 = 0;

    loop {
        print!("Input your guess: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // update the counter once the guess is made
        count += 1;

        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! it took you {} tries.", count);
                break;
            }
        }
    }
}
