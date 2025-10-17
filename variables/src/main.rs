use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=101);
    println!("The secret number is: {}", secret_number);

    loop {
        let mut guss = String::new();

        io::stdin()
            .read_line(&mut guss)
            .expect("Failed to read line");
        let guss: u32 = match guss.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                return;
            }
        };

        match guss.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                return;
            }
        }
    }

    
}
