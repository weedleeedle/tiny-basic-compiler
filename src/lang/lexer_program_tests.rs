#![cfg(test)]
//! Tests the lexer on an entire program.

use crate::lang::{ast::Variable, token::{Keyword, Symbol}};

use super::*;

/// Tests if an input stream generates a stream of tokens equivalent to the `expected_output_stream`
fn test_lexer_on_input(input_stream: &str, expected_output_stream: &[Token]) -> bool
{
    let mut lexer = create_lexer();
    let lexer_iter = lexer.parse_stream(input_stream);
    let expected_output_iter = expected_output_stream.iter();
    for (input, output) in lexer_iter.zip(expected_output_iter)
    {
        // Return false if we get any errors.
        if input.is_err()
        {
            println!("Got an error!");
            return false;
        }

        let input = input.unwrap();
        if &input != output
        {
            println!("Mismatched tokens, expected {:?}, got {:?}", output, input);
            return false;
        }
    }
    return true;
}

#[test]
fn test_lexer_on_hello_world()
{
    let input = "10 CLEAR\n\
                 20 PRINT \"What is your name?\"\n\
                 30 INPUT A\n\
                 40 PRINT \"Hello, \", A";
    println!("{:?}", input);
    let variable = Variable::try_from(b'A').unwrap();
    let expected_output: Vec<Token> = vec![
        Token::Number(10),
        Token::Keyword(Keyword::Clear),
        Token::NewLine,
        Token::Number(20),
        Token::Keyword(Keyword::Print),
        Token::String(String::from("What is your name?")),
        Token::NewLine,
        Token::Number(30),
        Token::Keyword(Keyword::Input),
        Token::Variable(variable),
        Token::NewLine,
        Token::Number(40),
        Token::Keyword(Keyword::Print),
        Token::String(String::from("Hello, ")),
        Token::Symbol(Symbol::Comma),
        Token::Variable(variable),
    ];

    assert!(test_lexer_on_input(input, &expected_output));
}
