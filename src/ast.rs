#[allow(dead_code)]
#[derive(Debug)]
pub enum ASTNode {
    Program {
        variable_section: Vec<VariableDeclaration>,
        statement_section: Vec<Statement>,
    },
    VariableDeclaration {
        names: Vec<String>,
        type_name: String,
    },
    Assignment {
        variable: String,
        expression: Expression,
    },
    Print {
        items: Vec<Expression>,
    },
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct VariableDeclaration {
    pub names: Vec<String>,
    pub type_name: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum Statement {
    Assignment(String, Expression),
    Print(Vec<Expression>),
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum Expression {
    IntegerLiteral(i32),
    Variable(String),
    StringLiteral(String), // Add this variant
    BinaryOperation {
        left: Box<Expression>,
        operator: char,
        right: Box<Expression>,
    },
}