use std::io;


fn main() {
    println!("Guess the number!");

    let mut guss = String::new();

    io::stdin()
    .read_line(&mut guss)
    .expect("Failed to read line");


    println!("You guessed: {}", guss);

}