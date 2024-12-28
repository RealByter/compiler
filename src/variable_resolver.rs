use crate::parser::*;
use std::collections::HashMap;

struct VariableEntry {
    unique_name: String,
    from_current_block: bool,
}
type VariableMap = HashMap<String, VariableEntry>;

pub fn resolve_variables(mut program: Program) -> Result<Program, String> {
    let mut variable_map: VariableMap = HashMap::new();
    program.function.body = resolve_block(program.function.body, &mut variable_map)?;
    Ok(program)
}

fn resolve_block(block: Block, variable_map: &mut VariableMap) -> Result<Block, String> {
    let mut body: Vec<BlockItem> = Vec::new();

    for block_item in block {
        body.push(match block_item {
            BlockItem::D(declaration) => {
                BlockItem::D(resolve_declaration(declaration, variable_map)?)
            }
            BlockItem::S(statement) => BlockItem::S(resolve_statement(statement, variable_map)?),
        })
    }

    Ok(body)
}

fn resolve_declaration(
    declaration: Declaration,
    variable_map: &mut VariableMap,
) -> Result<Declaration, String> {
    let mut expression: Option<Expression> = None;
    let name = match declaration {
        Declaration::Initialized(name, exp) => {
            expression = Some(exp);
            name
        }
        Declaration::Uninitialized(name) => name,
    };

    if variable_map.contains_key(&name) && variable_map.get(&name).unwrap().from_current_block {
        return Err("Duplicate variable declaration.".to_string());
    }

    let unique_name = make_unique_name(name.clone());
    variable_map.insert(
        name,
        VariableEntry {
            unique_name: unique_name.clone(),
            from_current_block: true,
        },
    );
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
    variable_map: &mut VariableMap,
) -> Result<Expression, String> {
    match expression {
        Expression::Assignment(op, left, right) => {
            match *left {
                Expression::Var(_) => {}
                _ => return Err("Invalid lvalue".to_string()),
            };
            Ok(Expression::Assignment(
                op,
                Box::new(resolve_expression(*left, variable_map)?),
                Box::new(resolve_expression(*right, variable_map)?),
            ))
        }
        Expression::Var(name) => {
            if variable_map.contains_key(&name) {
                Ok(Expression::Var(
                    variable_map.get(&name).unwrap().unique_name.clone(),
                ))
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
        Expression::Conditional(left, middle, right) => Ok(Expression::Conditional(
            Box::new(resolve_expression(*left, variable_map)?),
            Box::new(resolve_expression(*middle, variable_map)?),
            Box::new(resolve_expression(*right, variable_map)?),
        )),
    }
}

fn resolve_statement(
    statement: Statement,
    variable_map: &mut VariableMap,
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
        Statement::If(cond, if_body, else_body) => Ok(Statement::If(
            resolve_expression(cond, variable_map)?,
            Box::new(resolve_statement(*if_body, variable_map)?),
            match else_body {
                Some(else_body) => Some(Box::new(resolve_statement(*else_body, variable_map)?)),
                None => None,
            },
        )),
        Statement::Null => Ok(Statement::Null),
        Statement::Compound(block) => {
            let mut new_scope_variables = copy_variable_map(variable_map);
            Ok(Statement::Compound(resolve_block(
                block,
                &mut new_scope_variables,
            )?))
        }
        Statement::Break(label) => Ok(Statement::Break(label)),
        Statement::Continue(label) => Ok(Statement::Continue(label)),
        Statement::While(cond, body, label) => Ok(Statement::While(
            resolve_expression(cond, variable_map)?,
            Box::new(resolve_statement(*body, variable_map)?),
            label,
        )),
        Statement::DoWhile(body, cond, label) => Ok(Statement::DoWhile(
            Box::new(resolve_statement(*body, variable_map)?),
            resolve_expression(cond, variable_map)?,
            label,
        )),
        Statement::For(init, cond, post, body, label) => {
            let mut new_scope_variables = copy_variable_map(variable_map);
            Ok(Statement::For(
                resolve_for_init(init, &mut new_scope_variables)?,
                match cond {
                    Some(cond) => Some(resolve_expression(cond, &mut new_scope_variables)?),
                    None => None,
                },
                match post {
                    Some(post) => Some(resolve_expression(post, &mut new_scope_variables)?),
                    None => None,
                },
                Box::new(resolve_statement(*body, &mut new_scope_variables)?),
                label
            ))
        }
    }
}

fn resolve_for_init(init: ForInit, variable_map: &mut VariableMap) -> Result<ForInit, String> {
    match init {
        ForInit::InitDeclaration(declaration) => Ok(ForInit::InitDeclaration(resolve_declaration(
            declaration,
            variable_map,
        )?)),
        ForInit::InitExpression(expression) => {
            if let Some(expression) = expression {
                Ok(ForInit::InitExpression(Some(resolve_expression(
                    expression,
                    variable_map,
                )?)))
            } else {
                Ok(ForInit::InitExpression(None))
            }
        }
    }
}

fn copy_variable_map(variable_map: &VariableMap) -> VariableMap {
    let mut new_map: VariableMap = HashMap::new();
    for (key, value) in variable_map.iter() {
        new_map.insert(
            key.clone(),
            VariableEntry {
                unique_name: value.unique_name.clone(),
                from_current_block: false,
            },
        );
    }
    new_map
}

static mut USER_COUNTER: i64 = -1;

fn make_unique_name(name: String) -> String {
    format!("{}.u{}", name, unsafe {
        USER_COUNTER += 1;
        USER_COUNTER
    })
}
