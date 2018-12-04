extern crate k_rust;
use k_rust::lexer;

#[test]
fn lexer_word() {
    let word = "test";
    let tokens = lexer::tokenize(word);
    let res: bool;
    
    match tokens[0] {
        lexer::Token::Ident(ref _i) => res = true,
        _ =>  res = false,
    }
    assert_eq!(res, true)
}
