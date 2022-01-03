use std::io;
use rand::Rng;
use ferris_says::say;
use std::cmp::Ordering;
use std::io::{stdout, BufWriter, Read};
use libc::printf;

extern crate libc;

extern {
    fn list_file(input: libc::c_int) -> libc::c_int;
}

fn run_say() {
    let x = 5;
    let y = 10;
    println!("x = {} and y = {}", x, y);
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    let input = 4;
    let output = unsafe { list_file(input) };
    println!("{} * 2 = {}", input, output);
}

fn guess_number() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number");
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!")
    }
}

fn main() {
    println!("Hello, world!");

    // run_say();
    guess_number();
}
