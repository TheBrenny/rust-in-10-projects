#![allow(non_snake_case)]

use rand::Rng;
use std::usize;

// This is a constant!
pub const DEFAULT_SIZE: usize = 20;

// This is how you can namespace particular things.
// To namespace everything in the file, you would use 'pub mod namespace;'
pub mod Charset {
    pub struct Charset(pub &'static str);
    pub const UPPER: Charset = Charset("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    pub const LOWER: Charset = Charset("abcdefghijklmnopqrstuvwxyz");
    pub const DIGITS: Charset = Charset("0123456789");
    pub const HEX: Charset = Charset("0123456789ABCDEF");
    pub const ALL: Charset =
        Charset("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
}

// Simple fn
fn printUsage() {
    println!("");
    println!("Usage: token [<number> | <option>]");
    println!("");
    println!("Options: ");
    println!("    <number>");
    println!("          Generates a token of the parsed length");
    println!("    --guid");
    println!("    -g");
    println!("          Generates a GUID formatted token");
    println!("    --help");
    println!("    -h");
    println!("          Displays this help message");
}

fn getRandomString(size: usize, charset: Charset::Charset) -> String {
    // Create a new string that is mutable!
    let mut token = String::with_capacity(size);
    let mut rand = rand::thread_rng();

    for _ in 0..size {
        // Generate a random number between 0 and the length of the charset
        // It's important to remember to read the docs for the version you're using
        // and undestand what type you're using. gen_range takes a range. In Rust,
        // Ranges are expressed as start..end
        let rand_char = rand.gen_range(0..charset.0.len());
        token.push(charset.0.chars().nth(rand_char).unwrap());
    }

    return token;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        println!("{}", getRandomString(DEFAULT_SIZE, Charset::ALL));
    } else {
        let arg = &args[1];
        if arg == "--guid" || arg == "-g" {
            let mut guid = getRandomString(32, Charset::HEX);
            // I went reverse order because then we don't fk with math
            guid.insert_str(20, "-");
            guid.insert_str(16, "-");
            guid.insert_str(12, "-");
            guid.insert_str(8, "-");
            println!("{}", guid);
        } else if arg == "--help" || arg == "-h" {
            printUsage();
        } else if let Ok(num) = arg.parse::<usize>() {
            // The above is a neat feature of Rust. It's called
            // pattern matching, and it tries to match the result
            // of a function call to the left hand side.
            // If it doesn't work, then it skips the conditional block
            println!("{}", getRandomString(num, Charset::ALL));
        } else {
            eprintln!("Bad command. Use --help or -h for help");
        }
    }
}
