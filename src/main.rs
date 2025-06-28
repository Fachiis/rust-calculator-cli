use std::io::{self, Write};

use calculator_cli::{evaluate_expression, parse_expression, print_help_doc};

fn main() {
    println!("\nWelcome to the Calculator CLI project\n");
    print_help_doc();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        let lines = std::io::stdin().read_line(&mut user_input);

        match lines {
            Ok(_) => {
                let args: Vec<&str> = user_input.trim().split_whitespace().collect();

                if args.is_empty() {
                    continue;
                }

                match args[0].to_lowercase().as_str() {
                    "quit" | "q" => {
                        println!("Goodbye!");
                        break;
                    }
                    "help" => {
                        print_help_doc();
                    }
                    _ => match parse_expression(args.clone()) {
                        Ok(tokens) => {
                            match evaluate_expression(&*tokens) {
                                Ok(result) => {
                                    println!("Result: {}", result);
                                }
                                Err(e) => {
                                    println!("Error evaluating expression: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                            continue;
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error reading input: {}", e);
                continue;
            }
        }
    }
}
