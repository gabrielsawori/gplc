#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(String),
    IntLiteral(i64),
    StringLiteral(String),
    BoolLiteral(bool),
    InfixOp(Box<Expression>, String, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    LetStatement {
        name: String,
        is_mut: bool,
        value: Expression,
    },
    AssignStatement {
        name: String,
        value: Expression,
    },
    ReturnStatement(Expression),
    ExpressionStatement(Expression),
    FunctionDeclaration {
        name: String,
        params: Vec<String>,
        return_type: String,
        body: Vec<Statement>,
    },
    IfStatement {
        condition: Expression,
        body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    },
    WhileStatement {
        condition: Expression,
        body: Vec<Statement>,
    },
}
