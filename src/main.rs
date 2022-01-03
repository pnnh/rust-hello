use std::io;
use rand::Rng;
use ferris_says::say;
use std::cmp::Ordering;
use std::io::{stdout, BufWriter, Read};
use libc::{printf, pselect};

extern crate libc;

extern {
    fn list_file(input: libc::c_int) -> libc::c_int;
}

const MAX_POINTS: u32 = 100_000;

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
    let x = 5;
    // x = 6;      // 将编译出错，不可变变量无法再次赋值
    println!("Guess the number!");
    let mut y = 5;
    y = 6;      // 可以编译通过

    let secret_number = rand::thread_rng().gen_range(1..101);
    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        //let guess: u32 = guess.trim().parse().expect("Please type a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn run_tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("{} {} {}", five_hundred, six_point_four, one);
}

fn run_array() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    let index = 10;
    //let element = a[index]; // 将执行出错
    //println!("The value of element is: {}", element);
}

fn main() {
    println!("Hello, world!");

    // run_say();
    //guess_number();
    run_tuple();
}
