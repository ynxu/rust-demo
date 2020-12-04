use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 10);
    loop {
        println!("please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        // let guess: u32 = guess.trim().parse().expect("please type a number");
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("please type a number !");
                continue;
            }
        };

        println!("you guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("big"),
            Ordering::Equal => {
                println!("secret number is {}", secret_number);
                println!("win");
                break;
            },
        }
    }

}
