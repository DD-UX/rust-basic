pub mod game_context {
    use rand::Rng;
    use std::cmp::Ordering;
    use std::io;

    // This function is the game context
    pub fn init() {
        let mut agree_continue = String::new();

        'game_loop: loop {
            println!("Guess the number, human");

            let secret_number = rand::thread_rng().gen_range(1..=100);

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

            println!("Would you like to continue? y/n");
            io::stdin()
                .read_line(&mut agree_continue)
                .expect("answer not recorded");

            if !agree_continue.trim().to_lowercase().eq("y") {
                break 'game_loop;
            } else {
                agree_continue = String::new();
            }
        }
    }
}