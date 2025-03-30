use crate::lexer::Token;
use crate::ast::{ASTNode, VariableDeclaration, Statement, Expression};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<ASTNode, String> {
        self.parse_program().map_err(|e| format!("Error at token {:?}: {}", self.peek(), e))
    }

    fn parse_program(&mut self) -> Result<ASTNode, String> {
        self.expect_keyword("var")?;
        let variable_section = self.parse_variable_section()?;
        self.expect_keyword("begin")?;
        let statement_section = self.parse_statement_section()?;
        self.expect_keyword("end")?;
        Ok(ASTNode::Program {
            variable_section,
            statement_section,
        })
    }

    fn parse_variable_section(&mut self) -> Result<Vec<VariableDeclaration>, String> {
        let mut declarations = Vec::new();
        while self.peek() != Some(&Token::Keyword("begin".to_string())) {
            let names = self.parse_identifier_list()?;
            self.expect_symbol(':')?;
            let type_name = self.expect_keyword("integer").map(|_| "integer".to_string())?;
            self.expect_symbol(';')?;
            declarations.push(VariableDeclaration { names, type_name });
        }
        Ok(declarations)
    }

    fn parse_statement_section(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();
        while self.peek() != Some(&Token::Keyword("end".to_string())) {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        if let Some(Token::Identifier(var)) = self.peek().cloned() {
            self.consume(); // Consume the identifier
            self.expect_assignment()?; // Expect :=
            let expression = self.parse_expression()?; // Parse the expression
            self.expect_symbol(';')?; // Expect and consume the semicolon
            return Ok(Statement::Assignment(var, expression));
        }

        if self.match_keyword("writeln") {
            self.consume(); // Consume 'writeln'
            self.expect_symbol('(')?; // Expect opening parenthesis
            let items = self.parse_expression_list()?; // Parse the list of expressions
            self.expect_symbol(')')?; // Expect closing parenthesis
            self.expect_symbol(';')?; // Expect and consume the semicolon
            return Ok(Statement::Print(items));
        }

        Err(format!("Unexpected statement: {:?}", self.peek()))
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_term()?;
        while let Some(op) = self.peek().and_then(|token| {
            if let Token::Symbol(op) = token {
                Some(*op)
            } else {
                None
            }
        }) {
            if op == '+' || op == '-' || op == '*' || op == '/' {
                self.consume(); // Consume the operator
                let right = self.parse_term()?; // Parse the right-hand side
                left = Expression::BinaryOperation {
                    left: Box::new(left),
                    operator: op,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expression, String> {
        if let Some(token) = self.consume() {
            match token {
                Token::IntegerLiteral(value) => Ok(Expression::IntegerLiteral(value)),
                Token::Identifier(name) => Ok(Expression::Variable(name)),
                Token::StringLiteral(text) => Ok(Expression::StringLiteral(text)),
                _ => Err(format!("Unexpected term: {:?}", token)),
            }
        } else {
            Err("Unexpected end of input while parsing term".to_string())
        }
    }

    fn parse_expression_list(&mut self) -> Result<Vec<Expression>, String> {
        let mut expressions = Vec::new();
        expressions.push(self.parse_expression()?); // Parse the first expression
        while let Some(Token::Symbol(',')) = self.peek() {
            self.consume(); // Consume the comma
            expressions.push(self.parse_expression()?); // Parse the next expression
        }
        Ok(expressions)
    }

    // Utility functions
    fn expect_keyword(&mut self, keyword: &str) -> Result<(), String> {
        match self.consume() {
            Some(Token::Keyword(k)) if k == keyword => Ok(()),
            _ => Err(format!("Expected keyword '{}'", keyword)),
        }
    }

    fn expect_symbol(&mut self, symbol: char) -> Result<(), String> {
        match self.consume() {
            Some(Token::Symbol(s)) if s == symbol => Ok(()),
            _ => Err(format!("Expected symbol '{}'", symbol)),
        }
    }

    fn expect_assignment(&mut self) -> Result<(), String> {
        match self.consume() {
            Some(Token::Assignment) => Ok(()),
            _ => Err("Expected assignment operator ':='".to_string()),
        }
    }

    fn match_keyword(&self, keyword: &str) -> bool {
        matches!(self.peek(), Some(Token::Keyword(k)) if k == keyword)
    }

    fn parse_identifier_list(&mut self) -> Result<Vec<String>, String> {
        let mut names = Vec::new();
        if let Some(Token::Identifier(name)) = self.consume() {
            names.push(name);
        } else {
            return Err("Expected identifier".to_string());
        }
        while let Some(Token::Symbol(',')) = self.peek() {
            self.consume(); // Consume ','
            if let Some(Token::Identifier(name)) = self.consume() {
                names.push(name);
            } else {
                return Err("Expected identifier after ','".to_string());
            }
        }
        Ok(names)
    }

    fn consume(&mut self) -> Option<Token> {
        if self.current < self.tokens.len() {
            self.current += 1;
            Some(self.tokens[self.current - 1].clone())
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    #[test]
    fn test_parser_with_simple_program() {
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
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let expected_ast = ASTNode::Program {
            variable_section: vec![
                VariableDeclaration {
                    names: vec!["a".to_string(), "b".to_string()],
                    type_name: "integer".to_string(),
                },
                VariableDeclaration {
                    names: vec!["result".to_string()],
                    type_name: "integer".to_string(),
                },
            ],
            statement_section: vec![
                Statement::Assignment(
                    "a".to_string(),
                    Expression::IntegerLiteral(10),
                ),
                Statement::Assignment(
                    "b".to_string(),
                    Expression::IntegerLiteral(20),
                ),
                Statement::Assignment(
                    "result".to_string(),
                    Expression::BinaryOperation {
                        left: Box::new(Expression::Variable("a".to_string())),
                        operator: '+',
                        right: Box::new(Expression::Variable("b".to_string())),
                    },
                ),
                Statement::Print(vec![
                    Expression::StringLiteral("The result is: ".to_string()),
                    Expression::Variable("result".to_string()),
                ]),
            ],
        };

        assert_eq!(format!("{:?}", ast), format!("{:?}", expected_ast)); 
    }
}