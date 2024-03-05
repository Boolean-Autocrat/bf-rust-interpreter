use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("you need to provide the file name as an argument!");
        std::process::exit(1);
    }
    let filename = &args[1];
    if !filename.ends_with(".bf") {
        println!("you need to provide a file ending in .bf as an argument!");
        std::process::exit(1);
    }

    let mut file = File::open(filename).expect("unable to open file!");

    let mut src_code = String::new();
    file.read_to_string(&mut src_code)
        .expect("unable to read file");

    // memory tape init
    let mut tape = vec![0u8; 30000];
    let mut ptr = 0usize;

    let mut instr_ptr = 0usize;

    while instr_ptr < src_code.len() {
        match src_code.chars().nth(instr_ptr).unwrap() {
            '>' => ptr += 1,
            '<' => ptr -= 1,
            '+' => tape[ptr] = tape[ptr].wrapping_add(1),
            '-' => tape[ptr] = tape[ptr].wrapping_sub(1),
            '.' => print!("{}", tape[ptr] as char),
            ',' => {
                let mut input = [0u8; 1];
                std::io::stdin()
                    .read_exact(&mut input)
                    .expect("Unable to read input");
                tape[ptr] = input[0];
            }
            // TODO: fix loop handling
            '[' => {
                if tape[ptr] == 0 {
                    let mut count = 1;
                    while count > 0 {
                        instr_ptr += 1;
                        match src_code.chars().nth(instr_ptr).unwrap() {
                            '[' => count += 1,
                            ']' => count -= 1,
                            _ => {}
                        }
                    }
                }
            }
            ']' => {
                if tape[ptr] != 0 {
                    let mut count = 1;
                    while count > 0 {
                        instr_ptr -= 1;
                        match src_code.chars().nth(instr_ptr).unwrap() {
                            '[' => count -= 1,
                            ']' => count += 1,
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
        instr_ptr += 1;
    }
}

// TODO: Implement a lexer and parser separately

// #[derive(Debug, PartialEq)]
// enum OperCode {
//     MoveRight,
//     MoveLeft,
//     Increment,
//     Decrement,
//     Output,
//     Input,
//     LoopStart,
//     LoopEnd,
// }

// fn lex(source: String) -> Vec<OperCode> {
//     let mut tokens = Vec::new();
//     for c in source.chars() {
//         match c {
//             '>' => tokens.push(OperCode::MoveRight),
//             '<' => tokens.push(OperCode::MoveLeft),
//             '+' => tokens.push(OperCode::Increment),
//             '-' => tokens.push(OperCode::Decrement),
//             '.' => tokens.push(OperCode::Output),
//             ',' => tokens.push(OperCode::Input),
//             '[' => tokens.push(OperCode::LoopStart),
//             ']' => tokens.push(OperCode::LoopEnd),
//             _ => {}
//         }
//     }
//     tokens
// }
