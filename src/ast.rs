use crate::lexer::Token;

pub type Ast = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
    CreateTable,
    Select(SelectStatement),
    Insert(InsertStatement),
}

#[derive(Debug)]
pub struct ColumnDefinition {
    pub name: Token,
    pub datatype: Token,
}

#[derive(Debug)]
pub struct CreateTableStatement {
    pub name: Token,
    pub cols: Vec<ColumnDefinition>,
}

#[derive(Debug)]
pub struct SelectStatement {
    pub items: Vec<Expression>,
    pub from: Token,
}

impl SelectStatement {
    pub fn new(items: Vec<Expression>, from: Token) -> Self {
        Self { items, from }
    }
}

#[derive(Debug)]
pub struct InsertStatement {
    pub table_name: Token,
    pub values: Vec<Expression>,
}

impl InsertStatement {
    pub fn new(table_name: Token, values: Vec<Expression>) -> Self {
        Self { table_name, values }
    }
}

#[derive(Debug)]
pub enum Expression {
    Literal(LiteralExpression),
}

#[derive(Debug)]
pub struct LiteralExpression {
    pub literal: Token,
}

impl LiteralExpression {
    pub fn new(literal: Token) -> Self {
        Self { literal }
    }
}
