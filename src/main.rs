use std::io;

fn main() {
    println!("Welcome to the Calculator!");

    loop {
        println!("Enter first number: ");
        let mut a = String::new();
        if io::stdin().read_line(&mut a).is_err() {
            println!("Failed to read line");
            continue;
        }
        let a_trim = a.trim();
        if a_trim.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            std::process::exit(0);
        }
        let a_num: f64 = match a_trim.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        println!("Enter operator (+ - * / %): ");
        let mut op = String::new();
        if io::stdin().read_line(&mut op).is_err() {
            println!("Failed to read line");
            continue;
        }
        let op_trim = op.trim();
        if op_trim.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            std::process::exit(0);
        }
        if op_trim.is_empty() {
            println!("Please enter an operator.");
            continue;
        }

        println!("Enter second number: ");
        let mut b = String::new();
        if io::stdin().read_line(&mut b).is_err() {
            println!("Failed to read line");
            continue;
        }
        let b_trim = b.trim();
        if b_trim.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            std::process::exit(0);
        }
        let b_num: f64 = match b_trim.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        let result = match op_trim {
            "+" => Some(a_num + b_num),
            "-" => Some(a_num - b_num),
            "*" | "x" | "X" => Some(a_num * b_num),
            "/" => {
                if b_num == 0.0 {
                    println!("Error: division by zero");
                    None
                } else {
                    Some(a_num / b_num)
                }
            }
            "%" => {
                if b_num == 0.0 {
                    println!("Error: modulo by zero");
                    None
                } else {
                    Some(a_num % b_num)
                }
            }
            _ => {
                println!("Unsupported operator: {}", op_trim);
                None
            }
        };

        if let Some(r) = result {
            println!("Result: {} {} {} = {}", a_num, op_trim, b_num, r);
        }
        break;
    }
}