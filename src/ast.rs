use crate::lexer::Token;

pub type Ast = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
    CreateTable(CreateTableStatement),
    Select(SelectStatement),
    Insert(InsertStatement),
}

#[derive(Debug)]
pub struct ColumnDefinition {
    pub name: Token,
    pub datatype: Token,
}

impl ColumnDefinition {
    pub fn new(name: Token, datatype: Token) -> Self {
        Self { name, datatype }
    }
}

#[derive(Debug)]
pub struct CreateTableStatement {
    pub name: Token,
    pub cols: Vec<ColumnDefinition>,
}

impl CreateTableStatement {
    pub fn new(name: Token, cols: Vec<ColumnDefinition>) -> Self {
        Self { name, cols }
    }
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
