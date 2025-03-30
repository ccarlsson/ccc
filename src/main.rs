mod lexer;
use lexer::{tokenize};

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
    for token in tokens {
        println!("{:?}", token);
    }
}