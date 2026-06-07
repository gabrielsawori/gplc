#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(String),
    IntLiteral(i64),
    StringLiteral(String),
    InfixOp(Box<Expression>, String, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    LetStatement {
        name: String,
        is_mut: bool,
        value: Expression,
    },
    ReturnStatement(Expression),
    ExpressionStatement(Expression),
    FunctionDeclaration {
        name: String,
        return_type: String,
        body: Vec<Statement>,
    },
}