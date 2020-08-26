use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_num=rand::thread_rng().gen_range(1, 101);
    loop{
        println!("Please input your guess.");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        
        let guess: u32 = guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_num){
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win!");
                break;
                }
        }
    }
    println!("The secret number is: {}", secret_num);
}
