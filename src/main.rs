use std::{env, error, process};

fn get_prefix_numbers(s: &str) -> (i32, &str) {
    let mut num = 0;
    for (i, c) in s.chars().enumerate() {
        match c.to_digit(10) {
            None => return (num, &s[i..]),
            Some(n) => {
                num *= 10;
                num += n as i32;
            }
        }
    }

    (num, &"")
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Invalid number of arguments.");
        process::exit(1);
    }

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    let (first_num, mut substr) = get_prefix_numbers(&args[1]);
    println!("    mov rax, {}", first_num);

    while substr.len() > 0 {
        match substr.as_bytes()[0] as char {
            '+' => {
                let (num, substr_) = get_prefix_numbers(&substr[1..]);
                substr = substr_;
                println!("    add rax, {}", num);
            }
            '-' => {
                let (num, substr_) = get_prefix_numbers(&substr[1..]);
                substr = substr_;
                println!("    sub rax, {}", num);
            }
            c => {
                eprintln!("Unexpected character: {}", c);
                process::exit(1);
            }
        }
    }
    println!("    ret");

    Ok(())
}
