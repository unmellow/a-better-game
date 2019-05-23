
// use self::rand::Rng;
use std::cmp::Ordering;
// use std::io;

//making a public funtion to start the guessing game
pub fn cmp_guess(guess: u16, secret_number: u16) -> bool {
    // let secret_number = rand::thread_rng().gen_range(1, difficulty + 1);

     let equal: bool;

    //did you guess the number?
    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("{} is To Small...", guess);
             equal = false;
        }
        Ordering::Greater => {
            println!("{} is too Big!", guess);
            equal = false;
        }
        Ordering::Equal => {
            println!("{} IS CORRECT!!", guess);
            equal = true;
        }
    }
    equal
}
