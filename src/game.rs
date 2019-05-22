
// use self::rand::Rng;
use std::cmp::Ordering;
// use std::io;

//making a public funtion to start the guessing game
pub fn start_game(guess: u16, secret_number: u16) -> bool {
    // let secret_number = rand::thread_rng().gen_range(1, difficulty + 1);

     let breakloopnow: bool;

    //did you guess the number?
    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("{} is To Small...", guess);
             breakloopnow = false;
        }
        Ordering::Greater => {
            println!("{} is too Big!", guess);
            breakloopnow = false;
        }
        Ordering::Equal => {
            println!("{} IS CORRECT!!", guess);
            breakloopnow = true;
        }
    }
    breakloopnow
}
