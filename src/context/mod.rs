use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct GameContext {}

impl GameContext {
    pub fn init() {
        let mut agree_continue = String::new();

        while agree_continue != "y" {
            println!("Guess the number, human");

            let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

            // Loop guessing until the user guesses correctly
            GameContext::guess_number(secret_number);

            GameContext::retry_game(&mut agree_continue);

        }
    }

    // Private methods

    // Guess number logic
    fn guess_number (secret_number: u32) {
        'guess_loop: loop {
            println!("Give it a try and input a name in between 1 and 100");

            let mut guessed_number = String::new();

            io::stdin()
                .read_line(&mut guessed_number)
                .expect("Input number failed");

            // Shadow guessed_number to parse it as 'unsigned 32'
            let guessed_number: u32 = match guessed_number.trim().parse() {
                Ok(num) => num,
                Err(_error) => continue 'guess_loop,
            };

            println!("Guessed number is {guessed_number}");

            // Compare guessed number with the secret number already generated
            match guessed_number.cmp(&secret_number) {
                Ordering::Less => println!("{guessed_number} is smaller"),
                Ordering::Greater => println!("{guessed_number} is bigger"),
                Ordering::Equal => {
                    println!("{guessed_number} is the right number");
                    break 'guess_loop;
                }
            }
        }
    }

    // Retry logic
    fn retry_game (agree_continue: &mut String) {
        println!("Would you like to continue? y/n");
        io::stdin()
            .read_line(agree_continue)
            .expect("answer not recorded");
    }
}

