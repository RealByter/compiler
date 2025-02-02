use crate::parser::*;
use std::collections::HashMap;

struct IdentifierEntry {
    unique_name: String,
    from_current_scope: bool,
    has_linkage: bool,
}

type IdentifierMap = HashMap<String, IdentifierEntry>;

pub fn resolve_identifiers(program: Program) -> Result<Program, String> {
    let mut identifier_map: IdentifierMap = HashMap::new();
    let mut new_declarations: Vec<Declaration> = Vec::new();
    for declaration in program.declarations {
        match declaration {
            Declaration::FuncDecl(function) => {
                new_declarations.push(Declaration::FuncDecl(resolve_function_declaration(
                    function,
                    &mut identifier_map,
                )?));
            }
            Declaration::VarDecl(variable) => {
                new_declarations.push(Declaration::VarDecl(
                    resolve_file_scope_variable_declaration(variable, &mut identifier_map)?,
                ));
            }
        }
    }
    Ok(Program {
        declarations: new_declarations,
    })
}

fn resolve_file_scope_variable_declaration(
    var_declaration: VariableDeclaration,
    identifier_map: &mut IdentifierMap,
) -> Result<VariableDeclaration, String> {
    identifier_map.insert(
        var_declaration.name.clone(),
        IdentifierEntry {
            unique_name: var_declaration.name.clone(),
            from_current_scope: true,
            has_linkage: true,
        },
    );
    Ok(var_declaration)
}

fn resolve_block(block: Block, identifier_map: &mut IdentifierMap) -> Result<Block, String> {
    let mut body: Vec<BlockItem> = Vec::new();

    for block_item in block {
        body.push(match block_item {
            BlockItem::D(declaration) => {
                BlockItem::D(resolve_declaration(declaration, identifier_map)?)
            }
            BlockItem::S(statement) => BlockItem::S(resolve_statement(statement, identifier_map)?),
        })
    }

    Ok(body)
}

fn resolve_param_declaration(
    param: String,
    identifier_map: &mut IdentifierMap,
) -> Result<String, String> {
    match resolve_local_variable_declaration(
        VariableDeclaration {
            name: param,
            init: None,
            storage_class: None,
        },
        identifier_map,
    ) {
        Ok(decl) => Ok(decl.name),
        Err(err) => Err(err),
    }
}

fn resolve_local_variable_declaration(
    var_declaration: VariableDeclaration,
    identifier_map: &mut IdentifierMap,
) -> Result<VariableDeclaration, String> {
    if identifier_map.contains_key(&var_declaration.name) {
        let prev_entry = identifier_map.get(&var_declaration.name).unwrap();
        if prev_entry.from_current_scope
            && !(prev_entry.has_linkage
                && var_declaration.storage_class == Some(StorageClass::Extern))
        {
            return Err("Conflicting local declarations".to_string());
        }
    }

    if var_declaration.storage_class == Some(StorageClass::Extern) {
        identifier_map.insert(
            var_declaration.name.clone(),
            IdentifierEntry {
                unique_name: var_declaration.name.clone(),
                from_current_scope: true,
                has_linkage: true,
            },
        );
        Ok(var_declaration)
    } else {
        let unique_name = make_unique_name(var_declaration.name.clone());
        identifier_map.insert(
            var_declaration.name.clone(),
            IdentifierEntry {
                unique_name: unique_name.clone(),
                from_current_scope: true,
                has_linkage: false,
            },
        );
        Ok(VariableDeclaration {
            name: unique_name,
            init: match var_declaration.init {
                Some(expression) => Some(resolve_expression(expression, identifier_map)?),
                None => None,
            },
            storage_class: var_declaration.storage_class,
        })
    }
}

fn resolve_function_declaration(
    function_declaration: FunctionDeclaration,
    identifier_map: &mut IdentifierMap,
) -> Result<FunctionDeclaration, String> {
    if identifier_map.contains_key(&function_declaration.name) {
        let prev_entry = identifier_map.get(&function_declaration.name).unwrap();
        if prev_entry.from_current_scope && !prev_entry.has_linkage {
            return Err("Duplicate declaration".to_string());
        }
    }

    identifier_map.insert(
        function_declaration.name.clone(),
        IdentifierEntry {
            unique_name: function_declaration.name.clone(),
            from_current_scope: true,
            has_linkage: true,
        },
    );

    let mut inner_map = copy_identifier_map(identifier_map);
    let mut new_params: Vec<String> = Vec::new();
    for param in function_declaration.params {
        new_params.push(resolve_param_declaration(param, identifier_map)?);
    }

    let new_body = match function_declaration.body {
        Some(body) => Some(resolve_block(body, &mut inner_map)?),
        None => None,
    };

    Ok(FunctionDeclaration {
        name: function_declaration.name,
        params: new_params,
        body: new_body,
        storage_class: function_declaration.storage_class,
    })
}

fn resolve_declaration(
    declaration: Declaration,
    identifier_map: &mut IdentifierMap,
) -> Result<Declaration, String> {
    match declaration {
        Declaration::VarDecl(var_declaration) => Ok(Declaration::VarDecl(
            resolve_local_variable_declaration(var_declaration, identifier_map)?,
        )),
        Declaration::FuncDecl(function_declaration) => {
            if let Some(_) = function_declaration.body {
                return Err(format!(
                    "Local function declaration can't have a body: {}",
                    function_declaration.name
                ));
            }
            if function_declaration.storage_class == Some(StorageClass::Static) {
                return Err(format!(
                    "Local function declaration can't be static: {}",
                    function_declaration.name
                ));
            }
            Ok(Declaration::FuncDecl(resolve_function_declaration(
                function_declaration,
                identifier_map,
            )?))
        }
    }
}

fn resolve_expression(
    expression: Expression,
    identifier_map: &mut IdentifierMap,
) -> Result<Expression, String> {
    match expression {
        Expression::Assignment(op, left, right) => {
            match *left {
                Expression::Var(_) => {}
                _ => return Err("Invalid lvalue".to_string()),
            };
            Ok(Expression::Assignment(
                op,
                Box::new(resolve_expression(*left, identifier_map)?),
                Box::new(resolve_expression(*right, identifier_map)?),
            ))
        }
        Expression::Var(name) => {
            if identifier_map.contains_key(&name) {
                Ok(Expression::Var(
                    identifier_map.get(&name).unwrap().unique_name.clone(),
                ))
            } else {
                Err("Undeclared variable".to_string())
            }
        }
        Expression::Binary(op, left, right) => Ok(Expression::Binary(
            op,
            Box::new(resolve_expression(*left, identifier_map)?),
            Box::new(resolve_expression(*right, identifier_map)?),
        )),
        Expression::Unary(op, exp) => Ok(Expression::Unary(
            op,
            Box::new(resolve_expression(*exp, identifier_map)?),
        )),
        Expression::Constant(imm) => Ok(Expression::Constant(imm)),
        Expression::Conditional(left, middle, right) => Ok(Expression::Conditional(
            Box::new(resolve_expression(*left, identifier_map)?),
            Box::new(resolve_expression(*middle, identifier_map)?),
            Box::new(resolve_expression(*right, identifier_map)?),
        )),
        Expression::FunctionCall(name, args) => {
            if identifier_map.contains_key(&name) {
                let unique_name = identifier_map.get(&name).unwrap().unique_name.clone();
                let mut new_args: Vec<Expression> = Vec::new();
                for arg in args {
                    new_args.push(resolve_expression(arg, identifier_map)?);
                }
                Ok(Expression::FunctionCall(unique_name, new_args))
            } else {
                Err(format!("Undeclared function: {}", name))
            }
        }
    }
}

fn resolve_statement(
    statement: Statement,
    identifier_map: &mut IdentifierMap,
) -> Result<Statement, String> {
    match statement {
        Statement::Return(expression) => Ok(Statement::Return(resolve_expression(
            expression,
            identifier_map,
        )?)),
        Statement::Expression(expression) => Ok(Statement::Expression(resolve_expression(
            expression,
            identifier_map,
        )?)),
        Statement::If(cond, if_body, else_body) => Ok(Statement::If(
            resolve_expression(cond, identifier_map)?,
            Box::new(resolve_statement(*if_body, identifier_map)?),
            match else_body {
                Some(else_body) => Some(Box::new(resolve_statement(*else_body, identifier_map)?)),
                None => None,
            },
        )),
        Statement::Null => Ok(Statement::Null),
        Statement::Compound(block) => {
            let mut new_scope_variables = copy_identifier_map(identifier_map);
            Ok(Statement::Compound(resolve_block(
                block,
                &mut new_scope_variables,
            )?))
        }
        Statement::Break(label) => Ok(Statement::Break(label)),
        Statement::Continue(label) => Ok(Statement::Continue(label)),
        Statement::While(cond, body, label) => Ok(Statement::While(
            resolve_expression(cond, identifier_map)?,
            Box::new(resolve_statement(*body, identifier_map)?),
            label,
        )),
        Statement::DoWhile(body, cond, label) => Ok(Statement::DoWhile(
            Box::new(resolve_statement(*body, identifier_map)?),
            resolve_expression(cond, identifier_map)?,
            label,
        )),
        Statement::For(init, cond, post, body, label) => {
            let mut new_scope_variables = copy_identifier_map(identifier_map);
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
                label,
            ))
        }
        Statement::Switch(cond, cases, default, label) => {
            let mut resolved_cases: Vec<Case> = Vec::new();
            for case in cases {
                resolved_cases.push(Case {
                    cond: case.cond,
                    body: resolve_statement(case.body, identifier_map)?,
                });
            }

            Ok(Statement::Switch(
                resolve_expression(cond, identifier_map)?,
                resolved_cases,
                match default {
                    None => None,
                    Some(body) => Some(Box::new(resolve_statement(*body, identifier_map)?)),
                },
                label,
            ))
        }
    }
}

fn resolve_for_init(init: ForInit, identifier_map: &mut IdentifierMap) -> Result<ForInit, String> {
    match init {
        ForInit::InitDeclaration(declaration) => Ok(ForInit::InitDeclaration(
            resolve_local_variable_declaration(declaration, identifier_map)?,
        )),
        ForInit::InitExpression(expression) => {
            if let Some(expression) = expression {
                Ok(ForInit::InitExpression(Some(resolve_expression(
                    expression,
                    identifier_map,
                )?)))
            } else {
                Ok(ForInit::InitExpression(None))
            }
        }
    }
}

fn copy_identifier_map(identifier_map: &IdentifierMap) -> IdentifierMap {
    let mut new_map: IdentifierMap = HashMap::new();
    for (key, value) in identifier_map.iter() {
        new_map.insert(
            key.clone(),
            IdentifierEntry {
                unique_name: value.unique_name.clone(),
                from_current_scope: false,
                has_linkage: value.has_linkage,
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
