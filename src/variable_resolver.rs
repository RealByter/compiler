use crate::parser::*;
use std::collections::HashMap;

pub fn resolve_variables(mut program: Program) -> Result<Program, String> {
    let mut variable_map: HashMap<String, String> = HashMap::new();
    let mut body: Vec<BlockItem> = Vec::new();

    for block_item in program.function.body {
        body.push(match block_item {
            BlockItem::D(declaration) => {
                BlockItem::D(resolve_declaration(declaration, &mut variable_map)?)
            }
            BlockItem::S(statement) => {
                BlockItem::S(resolve_statement(statement, &mut variable_map)?)
            }
        })
    }

    program.function.body = body;

    Ok(program)
}

fn resolve_declaration(
    declaration: Declaration,
    variable_map: &mut HashMap<String, String>,
) -> Result<Declaration, String> {
    let mut expression: Option<Expression> = None;
    let name = match declaration {
        Declaration::Initialized(name, exp) => {
            expression = Some(exp);
            name
        }
        Declaration::Uninitialized(name) => name,
    };

    if variable_map.contains_key(&name) {
        return Err("Duplicate variable declaration.".to_string());
    }

    let unique_name = make_unique_name(name.clone());
    variable_map.insert(name, unique_name.clone());
    if let Some(exp) = expression {
        return Ok(Declaration::Initialized(
            unique_name,
            resolve_expression(exp, variable_map)?,
        ));
    } else {
        return Ok(Declaration::Uninitialized(unique_name));
    }
}

fn resolve_expression(
    expression: Expression,
    variable_map: &mut HashMap<String, String>,
) -> Result<Expression, String> {
    match expression {
        Expression::Assignment(left, right) => {
            match *left {
                Expression::Var(_) => {}
                _ => return Err("Invalid lvalid".to_string()),
            };
            Ok(Expression::Assignment(
                Box::new(resolve_expression(*left, variable_map)?),
                Box::new(resolve_expression(*right, variable_map)?),
            ))
        }
        Expression::Var(name) => {
            if variable_map.contains_key(&name) {
                Ok(Expression::Var(variable_map.get(&name).unwrap().clone()))
            } else {
                Err("Undeclared variable".to_string())
            }
        }
        Expression::Binary(op, left, right) => Ok(Expression::Binary(
            op,
            Box::new(resolve_expression(*left, variable_map)?),
            Box::new(resolve_expression(*right, variable_map)?),
        )),
        Expression::Unary(op, exp) => Ok(Expression::Unary(
            op,
            Box::new(resolve_expression(*exp, variable_map)?),
        )),
        Expression::Constant(imm) => Ok(Expression::Constant(imm)),
    }
}

fn resolve_statement(
    statement: Statement,
    variable_map: &mut HashMap<String, String>,
) -> Result<Statement, String> {
    match statement {
        Statement::Return(expression) => Ok(Statement::Return(resolve_expression(
            expression,
            variable_map,
        )?)),
        Statement::Expression(expression) => Ok(Statement::Expression(resolve_expression(
            expression,
            variable_map,
        )?)),
        Statement::Null => Ok(Statement::Null),
    }
}

static mut USER_COUNTER: i64 = -1;

fn make_unique_name(name: String) -> String {
    format!("{}.u{}", name, unsafe {
        USER_COUNTER += 1;
        USER_COUNTER
    })
}
