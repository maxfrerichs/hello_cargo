use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    let rand_num = rand::thread_rng().gen_range(1..101);
    let mut counter = 0;
    loop {
        println!("Guess the Number");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        match guess.cmp(&rand_num) {
            Ordering::Less => {
                counter = counter+1;
                println!("Too small!")
            },
            Ordering::Greater => {
                counter = counter+1;
                println!("Too big!")
            },
            Ordering::Equal => {
                println!("You win!");
                println!("Guesses needed: {}", counter);    
                break;
            }
        }
        println!("You guessed: {}", guess);
    }
}
