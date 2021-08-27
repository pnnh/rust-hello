use ferris_says::say;
use std::io::{stdout, BufWriter};
extern crate libc;

extern {
    fn list_file(input: libc::c_int) -> libc::c_int;
}

fn main() {
    println!("Hello, world!");

    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    let input = 4;
    let output = unsafe { list_file(input) };
    println!("{} * 2 = {}", input, output);
}
