use crate::parser::*;
use std::collections::HashMap;

#[derive(PartialEq)]
enum Type {
    Int,
    Function(usize), // param count
}

struct SymbolEntry {
    sym_type: Type,
    defined: bool,
}

type SymbolTable = HashMap<String, SymbolEntry>;

pub fn check_types(program: &Program) -> Result<(), String> {
    let mut symbol_table: SymbolTable = HashMap::new();
    for function in &program.functions {
        typecheck_function_declaration(&function, &mut symbol_table)?;
    }
    Ok(())
}

fn typecheck_function_declaration(
    func_declaration: &FunctionDeclaration,
    symbol_table: &mut SymbolTable,
) -> Result<(), String> {
    let func_type = Type::Function(func_declaration.params.len());
    let has_body = func_declaration.body.is_some();
    let mut already_defined = false;

    if symbol_table.contains_key(&func_declaration.name) {
        let old_decl = symbol_table.get(&func_declaration.name).unwrap();
        if old_decl.sym_type != func_type {
            return Err(format!(
                "Incompatible function declarations: {}",
                func_declaration.name
            ));
        } else {
            already_defined = old_decl.defined;
            if already_defined && has_body {
                return Err(format!(
                    "Function is defined more than once: {}",
                    func_declaration.name
                ));
            } else {
                Ok(())
            }
        }
    } else {
        symbol_table.insert(
            func_declaration.name.clone(),
            SymbolEntry {
                sym_type: func_type,
                defined: already_defined || has_body,
            },
        );

        if has_body {
            for param in &func_declaration.params {
                symbol_table.insert(
                    param.clone(),
                    SymbolEntry {
                        sym_type: Type::Int,
                        defined: false,
                    },
                );
                typecheck_block(func_declaration.body.as_ref().unwrap(), symbol_table)?
            }
        }

        Ok(())
    }
}

fn typecheck_block(block: &Block, symbol_table: &mut SymbolTable) -> Result<(), String> {
    for block_item in block {
        match block_item {
            BlockItem::D(declaration) => typecheck_declaration(declaration, symbol_table)?,
            BlockItem::S(statement) => typecheck_statement(statement, symbol_table)?,
        }
    }

    Ok(())
}

fn typecheck_declaration(
    declaration: &Declaration,
    symbol_table: &mut SymbolTable,
) -> Result<(), String> {
    match declaration {
        Declaration::VarDecl(var_declaration) => {
            typecheck_var_declaration(var_declaration, symbol_table)
        }
        Declaration::FuncDecl(func_declaration) => {
            typecheck_function_declaration(func_declaration, symbol_table)
        }
    }
}

fn typecheck_statement(
    statement: &Statement,
    symbol_table: &mut SymbolTable,
) -> Result<(), String> {
    match statement {
        Statement::Return(expression) => {
            typecheck_expression(expression, symbol_table)?;
        },
        Statement::Expression(expression) => {
            typecheck_expression(expression, symbol_table)?;
        },
        Statement::Null => {},
        Statement::If(expression, statement1, statement2) => {
            typecheck_expression(expression, symbol_table)?;
            typecheck_statement(statement1, symbol_table)?;
            if let Some(statement) = statement2 {
                typecheck_statement(statement, symbol_table)?;
            }
        },
        Statement::Compound(vec) => {
            typecheck_block(vec, symbol_table)?;
        },
        Statement::Break(_) => {},
        Statement::Continue(_) => {},
        Statement::While(expression, statement, _) => {
            typecheck_expression(expression, symbol_table)?;
            typecheck_statement(statement, symbol_table)?;
        },
        Statement::DoWhile(statement, expression, _) => {
            typecheck_statement(statement, symbol_table)?;
            typecheck_expression(expression, symbol_table)?;
        },
        Statement::For(for_init, expression1, expression2, statement, _) => {
            match for_init {
                ForInit::InitDeclaration(declaration) => {
                    typecheck_var_declaration(declaration, symbol_table)?;
                },
                ForInit::InitExpression(expression) => {
                    if let Some(expression) = expression {
                        typecheck_expression(expression, symbol_table)?;
                    }
                }
            }
            if let Some(expression) = expression1 {
                typecheck_expression(expression, symbol_table)?;
            }
            if let Some(expression) = expression2 {
                typecheck_expression(expression, symbol_table)?;
            }
            typecheck_statement(statement, symbol_table)?;
        },
        Statement::Switch(expression, vec, statement, _) => {
            typecheck_expression(expression, symbol_table)?;
            for case in vec {
                typecheck_statement(&case.body, symbol_table)?;
            }
            if let Some(statement) = statement {
                typecheck_statement(statement, symbol_table)?;
            }
        },
    }

    Ok(())
}

fn typecheck_expression(
    expression: &Expression,
    symbol_table: &mut SymbolTable,
) -> Result<(), String> {
    match expression {
        Expression::FunctionCall(func_name, args) => {
            if !symbol_table.contains_key(func_name) {
                return Err(format!("Function not declared: {}", func_name));
            }
            let func_type = &symbol_table.get(func_name).unwrap().sym_type;
            if *func_type == Type::Int {
                return Err(format!("Variable used as function name: {}", func_name));
            }
            if *func_type != Type::Function(args.len()) {
                return Err(format!(
                    "Function called with the wrong number of arguments: {}",
                    func_name
                ));
            }
            for arg in args {
                typecheck_expression(&arg, symbol_table)?;
            }
        }
        Expression::Var(var_name) => {
            if !symbol_table.contains_key(var_name) {
                return Err(format!("Variable not declared: {}", var_name));
            }

            if symbol_table.get(var_name).unwrap().sym_type != Type::Int {
                return Err(format!("Function used as variable name: {}", var_name));
            }
        }
        Expression::Constant(_) => {}
        Expression::Unary(_, expression) => {
            typecheck_expression(&expression, symbol_table)?;
        }
        Expression::Binary(_, expression1, expression2) => {
            typecheck_expression(&expression1, symbol_table)?;
            typecheck_expression(&expression2, symbol_table)?;
        }
        Expression::Assignment(_, expression1, expression2) => {
            typecheck_expression(&expression1, symbol_table)?;
            typecheck_expression(&expression2, symbol_table)?;
        },
        Expression::Conditional(expression1, expression2, expression3) => {
            typecheck_expression(&expression1, symbol_table)?;
            typecheck_expression(&expression2, symbol_table)?;
            typecheck_expression(&expression3, symbol_table)?;
        },
    }

    Ok(())
}

fn typecheck_var_declaration(
    var_declaration: &VariableDeclaration,
    symbol_table: &mut SymbolTable,
) -> Result<(), String> {
    symbol_table.insert(
        var_declaration.name.clone(),
        SymbolEntry {
            sym_type: Type::Int,
            defined: false,
        },
    );
    if let Some(exp) = &var_declaration.init {
        typecheck_expression(exp, symbol_table)?
    }
    Ok(())
}
