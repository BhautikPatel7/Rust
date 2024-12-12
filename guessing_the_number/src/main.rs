use rand::{self, Rng};
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess The Number!");
    println!("Hey! Plese guess a number between 1 to 100 : ");

    let secreatnumber = rand::thread_rng().gen_range(1..=1000);
    println!("generated screate number {secreatnumber}");

    // guess.push_str("my name is ");
    loop {
        println!("Please input your guess.");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failes to read an line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Please enter a valid input");
                continue;
            }
        }; //45 + one enter

        match guess.cmp(&secreatnumber) {
            Ordering::Equal => {
                println!("You wan");
                break;
            }
            Ordering::Greater => println!("to big"),
            Ordering::Less => println!("to small"),
        };
    }
}
