use std::{collections::LinkedList, env, error, process};

#[derive(PartialEq, Eq)]
enum TokenKind {
    TkReserved,
    TkNum,
    TkEof,
}

struct Token<'a> {
    kind: TokenKind,
    val: i32,
    token_str: &'a str,
}

impl<'a> Token<'a> {
    fn new(kind: TokenKind, val: i32, token_str: &'a str) -> Self {
        Self {
            kind,
            val,
            token_str,
        }
    }
}

fn consume<'a>(op: char, mut token: LinkedList<Token<'a>>) -> (bool, LinkedList<Token<'a>>) {
    if token.front().unwrap().kind != TokenKind::TkReserved
        || token.front().unwrap().token_str.as_bytes()[0] as char != op
    {
        return (false, token);
    }
    token.pop_front();
    (true, token)
}

fn expect<'a>(op: char, mut token: LinkedList<Token<'a>>) -> LinkedList<Token<'a>> {
    if token.front().unwrap().kind != TokenKind::TkReserved
        || token.front().unwrap().token_str.as_bytes()[0] as char != op
    {
        eprintln!("{} is expected.", op);
        process::exit(1);
    }
    token.pop_front();
    token
}

fn expect_number<'a>(mut token: LinkedList<Token<'a>>) -> (i32, LinkedList<Token<'a>>) {
    if token.front().unwrap().kind != TokenKind::TkNum {
        eprintln!("Number is expected.");
        process::exit(1);
    }
    let val: i32 = token.front().unwrap().val;
    token.pop_front();
    (val, token)
}

fn at_eof<'a>(token: &LinkedList<Token<'a>>) -> bool {
    token.front().unwrap().kind == TokenKind::TkEof
}

fn new_token<'a>(
    kind: TokenKind,
    mut cur: LinkedList<Token<'a>>,
    val: i32,
    token_str: &'a str,
) -> LinkedList<Token<'a>> {
    cur.push_back(Token::new(kind, val, token_str));
    cur
}

fn tokenize<'a>(mut s: &'a str) -> LinkedList<Token<'a>> {
    let mut token: LinkedList<Token<'a>> = LinkedList::new();

    while s.len() > 0 {
        match s.as_bytes()[0] as char {
            c if c.is_whitespace() => s = &s[1..],
            c if c == '+' || c == '-' => {
                token = new_token(TokenKind::TkReserved, token, 0, s);
                s = &s[1..];
            }
            c if c.is_digit(10) => {
                let (val, substr) = get_prefix_numbers(s);
                token = new_token(TokenKind::TkNum, token, val, s);
                s = substr;
            }
            _ => {
                eprintln!("Can't tokenize.");
                process::exit(1);
            }
        }
    }

    new_token(TokenKind::TkEof, token, 0, s)
}

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

fn output_assembly<'a>(token: LinkedList<Token<'a>>) -> () {
    if at_eof(&token) {
        return;
    }

    let (is_plus, new_token) = consume('+', token);
    if is_plus {
        let (val, subtoken) = expect_number(new_token);
        println!("    add rax, {}", val);
        output_assembly(subtoken);
    } else {
        let subtoken = expect('-', new_token);
        let (val, subtoken) = expect_number(subtoken);
        println!("    sub rax, {}", val);
        output_assembly(subtoken);
    }
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

    let token = tokenize(&args[1]);
    let (num, new_token) = expect_number(token);
    println!("    mov rax, {}", num);

    output_assembly(new_token);
    println!("    ret");

    Ok(())
}
