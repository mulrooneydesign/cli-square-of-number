use term_size::dimensions;

fn main() {
    loop {
        println!("\x1b[1;32m\nEnter a number to square:\x1b[0m");
        println!("Type 'exit' to close the program\n");

        let stdin = std::io::stdin();

        let mut user_input = String::new();
        stdin
            .read_line(&mut user_input)
            .expect("Failed to read user input");

        if user_input.trim() == "exit" {
            println!("Closing Program\nBye bye! 😆");
            break;
        }

        let number: i32 = match user_input.trim().parse::<i32>() {
            Ok(num) if num > 0 => num,
            _ => {
                println!(
                    "Sorry, but '{}' is not a number that can be squared",
                    user_input.trim()
                );
                continue;
            }
        };

        let terminal_width = "=".repeat(dimensions().unwrap().0);

        println!("\n\x1b[1;32m{}\x1b[0m", terminal_width);
        println!("-- {} is {} squared  --", number * number, number);
        println!("\x1b[1;32m{}\x1b[0m", terminal_width);
    }
}
