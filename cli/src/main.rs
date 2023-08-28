use std::process::exit;

use clap::{arg, Command};
use droll::{base_prng_engine, interpreter::eval, parser::parse};

fn main() {
    let cmd = Command::new("droll")
        .about("Parse dice notation and print the result")
        .arg(arg!(<DICE_NOTATION>))
        .arg_required_else_help(true);

    let matches = cmd.get_matches();

    let input = matches
        .get_one::<String>("DICE_NOTATION")
        .expect("Required input argument is missing");

    match parse(input) {
        Ok(parse_tree) => {
            println!("{}", eval(base_prng_engine)(parse_tree));
            exit(0);
        }
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };
}
