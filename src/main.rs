use std::{env, fs};

use buh::Expression;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("No file found")
    }

    let program = &args[1];
    let contents = fs::read_to_string(program).expect("Should have been able to read the file");
    let (exp, _) = Expression::new(&contents);

    println!("{:?}", exp);
}
