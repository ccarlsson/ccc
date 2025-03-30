mod lexer;
mod parser;
mod ast;


use crate::lexer::tokenize;
use crate::parser::Parser;


fn main() {
    let source_code = r#"
        var
            a, b: integer;
            result: integer;

        begin
            a := 10;
            b := 20;
            result := a + b;
            writeln('The result is: ', result);
        end
    "#;

    let tokens = tokenize(source_code);
    println!("{:?}", tokens); // Debugging: Print the tokens
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    println!("{:?}", ast);
}