extern crate k_rust;
use k_rust::lexer;

fn main() {
    let word = "hello";
    let tokens = lexer::tokenize(word);

    println!("{:?}", tokens[0]);
}
