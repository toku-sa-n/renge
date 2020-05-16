use std::{env, error, process};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Invalid number of arguments.");
        process::exit(1);
    }

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("    mov rax, {}", args[1].parse::<i32>()?);
    println!("    ret");

    Ok(())
}
