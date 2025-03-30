#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(String),       // e.g., "var", "begin", "end"
    Identifier(String),    // e.g., variable names like "a", "b"
    IntegerLiteral(i32),   // e.g., numbers like 10, 20
    StringLiteral(String), // e.g., "The result is: "
    Symbol(char),          // e.g., ':', ';', '(', ')'
    Assignment,            // e.g., :=
    EndOfInput,            // Represents the end of the input
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            // Keywords and identifiers
            'a'..='z' | 'A'..='Z' => {
                let mut word = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() {
                        word.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if word == "var" || word == "begin" || word == "end"  || word == "integer" || word == "writeln" {
                    tokens.push(Token::Keyword(word));
                } else {
                    tokens.push(Token::Identifier(word));
                }
            }
            // Numbers
            '0'..='9' => {
                let mut number = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_numeric() {
                        number.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::IntegerLiteral(number.parse::<i32>().unwrap()));
            }
            // String literals
            '"' => {
                chars.next(); // Consume the opening quote
                let mut string = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch == '"' {
                        chars.next(); // Consume the closing quote
                        break;
                    } else {
                        string.push(ch);
                        chars.next();
                    }
                }
                tokens.push(Token::StringLiteral(string));
            }
            '+' | '-' | '*' | '/' => {
                tokens.push(Token::Symbol(c));
                chars.next();
            }
            '\'' => {
                chars.next();
                let mut string = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch == '\'' {
                        break;
                    }
                    string.push(ch);
                    chars.next();
                }
                chars.next();
                tokens.push(Token::StringLiteral(string));
            }
            // Symbols and assignment
            ':' => {
                chars.next(); // Consume ':'
                if chars.peek() == Some(&'=') {
                    chars.next(); // Consume '='
                    tokens.push(Token::Assignment);
                } else {
                    tokens.push(Token::Symbol(':'));
                }
            }
            // Other symbols
            ';' | '(' | ')' | ',' => {
                tokens.push(Token::Symbol(c));
                chars.next();
            }
            // Whitespace
            _ if c.is_whitespace() => {
                chars.next(); // Skip whitespace
            }
            // Unrecognized characters
            _ => {
                panic!("Unexpected character: {}", c);
            }
        }
    }

    tokens.push(Token::EndOfInput);
    tokens
}

#[cfg(test)] // Marks this module as test-only
mod tests {
    use super::*; // Import the main module for access to Token and tokenize

    #[test]
    fn test_tokenize_simple_program() {
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

        
        // Expected tokens for the provided source code
        let expected_tokens = vec![
            Token::Keyword("var".to_string()),
            Token::Identifier("a".to_string()),
            Token::Symbol(','),
            Token::Identifier("b".to_string()),
            Token::Symbol(':'),
            Token::Keyword("integer".to_string()),
            Token::Symbol(';'),
            Token::Identifier("result".to_string()),
            Token::Symbol(':'),
            Token::Keyword("integer".to_string()),
            Token::Symbol(';'),
            Token::Keyword("begin".to_string()),
            Token::Identifier("a".to_string()),
            Token::Assignment,
            Token::IntegerLiteral(10),
            Token::Symbol(';'),
            Token::Identifier("b".to_string()),
            Token::Assignment,
            Token::IntegerLiteral(20),
            Token::Symbol(';'),
            Token::Identifier("result".to_string()),
            Token::Assignment,
            Token::Identifier("a".to_string()),
            Token::Symbol('+'),
            Token::Identifier("b".to_string()),
            Token::Symbol(';'),
            Token::Keyword("writeln".to_string()),
            Token::Symbol('('),
            Token::StringLiteral("The result is: ".to_string()),
            Token::Symbol(','),
            Token::Identifier("result".to_string()),
            Token::Symbol(')'),
            Token::Symbol(';'),
            Token::Keyword("end".to_string()),
            Token::EndOfInput,
        ];

        // for (actual, expected) in tokens.iter().zip(expected_tokens.iter()) {
        //     println!("Actual: {:?}, Expected: {:?}", actual, expected);
        // }

        assert_eq!(tokens, expected_tokens, "Token mismatch. Uncomment the printout and check the output for differences.");
    }
}