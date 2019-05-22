extern crate rand;

use self::rand::Rng;
use std::cmp::Ordering;
use std::io;

//making a public funtion to start the guessing game
pub fn start_game(difficulty: u16) {
    let secret_number = rand::thread_rng().gen_range(1, difficulty + 1);

    let mut dyg = false;
    //starting game loop
    loop {
        if dyg == true {
            println!("Guess again!");
        } else {
            println!("guess a number between 1 and {}.", difficulty);
            dyg = true;
        }

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line WHAT DID YOU DO");

        let guess: u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //did you guess the number?
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{} is To Small...", guess),
            Ordering::Greater => println!("{} is too Big!", guess),
            Ordering::Equal => {
                println!("{} IS CORRECT!!", guess);
                break;
            }
        }
    }
}
