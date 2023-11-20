mod cli;
mod lexer;
mod parser;
mod data_structures;
mod utils;

mod test;

use parser::parser::Parser;

fn main() {
    let json_file_content: String = cli::init();
    let mut parser = Parser::new(json_file_content);

    match parser.parse() {
        Ok(_) => println!("Your JSON file is valid!"),
        Err(_) => println!("Your JSON file is not valid!"),
    }
}

