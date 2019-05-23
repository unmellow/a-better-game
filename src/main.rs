extern crate rand;

use self::rand::Rng;
use std::io;

mod game;

fn main() {
    let mut user = String::new();

    println!("Hello sir! \nlet's start with introductions");
    println!("your name is? \n");

    io::stdin()
        .read_line(&mut user)
        .expect("there's something wrong with what you typed");
        //play loop
    loop {
        println!("\nHello {}my name is Julia   \nYou can say 'hi'   \nto leave say 'bye'   \nyou can give me a 'kiss'   \nonce you gave me your name,  \nwe can also 'play' a game    \n", user);
        
        let mut cmd = String::new();

        io::stdin()
            .read_line(&mut cmd)
            .expect("there's something wrong with what you typed");

            match cmd.trim().as_ref() {
            "bye" => {
                println!("\nbye");
                break;
            }
            "hi" | "hello" | "hey" => println!("\nhello {} ", user),
            "play" => {
                let mut played = false;
                loop {
                    if played == false {
                        println!("\nwant to play a guessing game? ");
                    } else if played == true {
                        println!("\nwant to play again? \n");
                    }
                    //seeing if you want to play the game before it starts
                    let mut wtpgg = String::new();

                    io::stdin()
                        .read_line(&mut wtpgg)
                        .expect("there's something wrong with what you typed");
                    match wtpgg.trim().as_ref() {
                        "yes" | "sure" => loop {
                            let mut difficulty = String::new();

                            println!("\nChoose the diffiuculty enter Number   \n");

                            io::stdin()
                                .read_line(&mut difficulty)
                                .expect("Failed to read line WHAT DID YOU DO");

                            let difficulty: u16 = match difficulty.trim().parse() {
                                Ok(num) => num,
                                Err(_) => continue,
                            };

                            if difficulty > 1 {
                                println!("\nGuess the number! \n");
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
                                    //creating a string that contains user input
                                    let mut guess = String::new();
                                    io::stdin()
                                        .read_line(&mut guess)
                                        .expect("Failed to read line WHAT DID YOU DO");

                                        //converting input string into a positive interger
                                        let guess: u16 = match guess.trim().parse() {
                                        Ok(num) => num,
                                        Err(_) => continue,
                                    };

                                    //checking if the awnser equals the secret number and breaking the loop afterwards
                                    let equal = game::cmp_guess(guess, secret_number);
                                    if equal == true {
                                        break;
                                    }
                                }
                                played = true;
                                break;
                            } else {
                                continue;
                            }
                        },
                        "no" | "nah" => break,
                        _ => println!("\nthat's not an awnser {}", user),
                    }
                }
            }
            "kiss" | "smouch" => println!("\n*smouch* \ni liked that {}", user),
            _ => println!("\ni don't understand"),
        }
    }
}
