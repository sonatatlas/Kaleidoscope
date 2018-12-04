extern crate k_rust;
use k_rust::lexer;

fn main() {
    let a = "hello";
    let res = lexer::tokenize(a);

    println!("{:?}", res);
}
