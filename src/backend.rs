use std::collections::HashMap;

use crate::{
    ast::{CreateTableStatement, InsertStatement, SelectStatement, Statement},
    lexer::Token,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Datatype {
    Int,
    Text,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(i64),
    Text(String),
}

#[derive(Debug)]
pub struct Column {
    pub name: String,
    pub datatype: Datatype,
}

impl Column {
    pub fn new(name: String, datatype: Datatype) -> Self {
        Self { name, datatype }
    }
}

#[derive(Debug)]
pub struct Table {
    pub columns: Vec<Column>,
    pub rows: Vec<Vec<Value>>,
}

impl Table {
    pub fn new(columns: Vec<Column>, rows: Vec<Vec<Value>>) -> Self {
        Self { columns, rows }
    }
}

#[derive(Debug)]
pub struct Database {
    pub tables: HashMap<String, Table>,
}

#[derive(Debug)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Value>>,
}

#[derive(Debug)]
pub enum ExecutionResult {
    Ok,
    Rows(QueryResult),
}

#[derive(Debug)]
pub struct ExecutionError {
    pub message: String,
    pub tokens: Vec<Token>,
}

impl ExecutionError {
    pub fn new(message: String, tokens: Vec<Token>) -> Self {
        Self { message, tokens }
    }
}

impl Database {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    pub fn execute(&mut self, stmt: Statement) -> Result<ExecutionResult, ExecutionError> {
        match stmt {
            Statement::CreateTable(stmt) => self.create_table(stmt),
            Statement::Insert(stmt) => self.insert(stmt),
            Statement::Select(stmt) => self.select(stmt),
        }
    }

    fn create_table(
        &mut self,
        stmt: CreateTableStatement,
    ) -> Result<ExecutionResult, ExecutionError> {
        let table_name = stmt.name.value.clone();

        if self.tables.contains_key(&table_name) {
            return Err(ExecutionError::new(
                format!("table '{}' already exists", table_name),
                vec![stmt.name.clone()],
            ));
        }

        let mut columns = Vec::new();
        for col in &stmt.cols {
            let datatype = match col.datatype.value.to_uppercase().as_str() {
                "INT" => Datatype::Int,
                "TEXT" => Datatype::Text,
                other => {
                    return Err(ExecutionError::new(
                        format!("unknoown datatype: {}", other.to_uppercase()),
                        vec![col.datatype.clone()],
                    ));
                }
            };

            columns.push(Column::new(col.name.value.clone(), datatype));
        }

        self.tables.insert(table_name, Table::new(columns, vec![]));

        Ok(ExecutionResult::Ok)
    }

    fn insert(&mut self, stmt: InsertStatement) -> Result<ExecutionResult, ExecutionError> {
        todo!();
    }

    fn select(&mut self, stmt: SelectStatement) -> Result<ExecutionResult, ExecutionError> {
        todo!();
    }
}
