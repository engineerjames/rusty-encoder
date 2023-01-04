use rusty_encoder::base64encoder;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Program usage: `rusty-encoder -f <path_to_file>`");
        println!("               `rusty-encoder <text_to_encode>");
        return;
    }

    let read_input_from_file = args.contains(&String::from("-f"));

    let encoded_string: String;
    if read_input_from_file {
        let file_binary_contents = fs::read(&args[2]).expect("Unable to read file!");
        encoded_string = base64encoder::encode(&file_binary_contents);
    } else {
        let string_as_bytes = args[1].as_bytes().to_vec();
        encoded_string = base64encoder::encode(&string_as_bytes);
    }

    println!("{}", encoded_string);
}
