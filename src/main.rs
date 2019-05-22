use std::io;

mod game;

fn main() {
    let mut user = String::new();

    println!("Hello sir! \nlet's start with introductions");
    println!("your name is? \n");

    io::stdin()
        .read_line(&mut user)
        .expect("there's something wrong with what you typed");

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
                                game::start_game(difficulty);
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
        /*
                if cmd.contains("bye") || cmd.contains("exit") == true {

                    println!("\nbye {}", user);
                        break;
                }

                if cmd.contains("hi") || cmd.contains("hello") || cmd.contains("hey") == true {

                    println!("\nhello {}  \n", user);
                }

                if cmd.contains("kiss") == true {
                    println!("\n*smouch*   \ni liked that {}  \n",user)
                }

        if cmd.contains("play") == true {
            let mut played = false;
            loop {
                if played == false {
                    println!("\nwant to play a guessing game? \n");
                } else if played == true {
                    println!("\nwant to play again?   \n");
                }

                let mut wtpgg = String::new();

                io::stdin()
                    .read_line(&mut wtpgg)
                    .expect("there's something wrong with what you typed");

                if wtpgg.contains("yes") || wtpgg.contains("sure") == true {
                    //the difficulty selection loop
                    loop {
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
                            game::start_game(difficulty);
                            played = true;
                            break;
                        } else {
                            continue;
                        }
                    }
                } else if wtpgg.contains("no") || wtpgg.contains("nah") == true {
                    break;
                } else {
                    println!("\nthat's not an answer  \n")
                }
            }
        }
        */

        /*
            if cmd.contains() == true {

            }

        */
    }
}
