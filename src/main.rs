use std::env;
use std::fs;

use rustybase::base64encoder::is_padding_necessary;
use rustybase::base64encoder::encode;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Program usage: `rustybase <path_to_file>`");
        return;
    }

    let file_binary_contents = fs::read(&args[1]).expect("Unable to read file!");
    println!("Size of file: {} bytes.", file_binary_contents.len());

    let encoded_string = encode(
        &file_binary_contents,
        is_padding_necessary(&file_binary_contents),
    );

    println!("{}", encoded_string);
}
