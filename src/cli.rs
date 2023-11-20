use std::fs;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the json file to validate
    #[arg(name="File")]
    my_flie: Option<String>,
}

pub fn init() -> String {

    let args = Args::parse();
    let mut text: String = "".to_string();

    if let Some(my_file) = args.my_flie.as_deref() {

        // check if the file extension is json
        if!my_file.ends_with(".json") {
            eprintln!("File extension must be json only");
        }

        text = fs::read_to_string(my_file).expect("error while reading your file, please try again and make sure the path is correct");
    }

    text
} 