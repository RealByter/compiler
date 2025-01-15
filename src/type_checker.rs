use crate::parser::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Type {
    Int,
    Function(usize), // param count
}

#[derive(Debug, PartialEq)]
pub struct SymbolEntry {
    sym_type: Type,
    defined: bool,
    identifier_attrs: IdentifierAttr,
}

#[derive(Debug, PartialEq)]
pub enum IdentifierAttr {
    FunAttr(bool, bool),            // defined, global
    StaticAttr(InitialValue, bool), // init, global
    LocalAttr,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum InitialValue {
    Tentative,
    Initial(i64),
    NoInitializer,
}

pub type SymbolTable = HashMap<String, SymbolEntry>;

pub fn check_types(program: &Program) -> Result<SymbolTable, String> {
    let mut symbol_table: SymbolTable = HashMap::new();
    for declaration in &program.declarations {
        match declaration {
            Declaration::FuncDecl(function) => {
                typecheck_function_declaration(&function, &mut symbol_table)?;
            }
            Declaration::VarDecl(var) => {
                typecheck_file_scope_variable_declaration(var, &mut symbol_table)?;
            }
        }
    }
    Ok(symbol_table)
}

fn typecheck_function_declaration(
    func_declaration: &FunctionDeclaration,
    symbol_table: &mut SymbolTable,
) -> Result<(), String> {
    let func_type = Type::Function(func_declaration.params.len());
    let has_body = func_declaration.body.is_some();
    let mut already_defined = false;
    let mut global = func_declaration.storage_class != Some(StorageClass::Static);

    if symbol_table.contains_key(&func_declaration.name) {
        let old_decl = symbol_table.get(&func_declaration.name).unwrap();
        if old_decl.sym_type != func_type {
            return Err(format!(
                "Incompatible function declarations: {}",
                func_declaration.name
            ));
        }

        already_defined = old_decl.defined;
        if already_defined && has_body {
            return Err(format!(
                "Function is defined more than once: {}",
                func_declaration.name
            ));
        }

        if (old_decl.identifier_attrs == IdentifierAttr::FunAttr(true, true)
            || old_decl.identifier_attrs == IdentifierAttr::FunAttr(false, true))
            && func_declaration.storage_class == Some(StorageClass::Static)
        {
            return Err(format!(
                "Static function declaration follows non-static: {}",
                func_declaration.name
            ));
        }
        global = true;
    }
    let attrs = IdentifierAttr::FunAttr(already_defined || has_body, global);
    symbol_table.insert(
        func_declaration.name.clone(),
        SymbolEntry {
            sym_type: func_type,
            defined: already_defined || has_body,
            identifier_attrs: attrs,
        },
    );
    if has_body {
        for param in &func_declaration.params {
            symbol_table.insert(
                param.clone(),
                SymbolEntry {
                    sym_type: Type::Int,
                    defined: false,
                    identifier_attrs: IdentifierAttr::LocalAttr,
                },
            );
        }
        typecheck_block(func_declaration.body.as_ref().unwrap(), symbol_table)?
    }

    Ok(())
}

fn typecheck_file_scope_variable_declaration(
    var_declaration: &VariableDeclaration,
    symbol_table: &mut SymbolTable,
) -> Result<(), String> {
    let mut initial_value = match var_declaration.init {
        Some(Expression::Constant(val)) => InitialValue::Initial(val),
        None if var_declaration.storage_class == Some(StorageClass::Extern) => {
            InitialValue::NoInitializer
        }
        None => InitialValue::Tentative,
        _ => return Err("Non-constant initializer".to_string()),
    };

    let mut global = var_declaration.storage_class != Some(StorageClass::Static);

    if let Some(old_decl) = symbol_table.get(&var_declaration.name) {
        if old_decl.sym_type != Type::Int {
            return Err("Function redeclared as variable".to_string());
        }

        match (&var_declaration.storage_class, &old_decl.identifier_attrs) {
            (Some(StorageClass::Extern), IdentifierAttr::StaticAttr(_, glob)) => {
                global = *glob;
            }
            (_, IdentifierAttr::StaticAttr(_, glob)) if global != *glob => {
                return Err("Conflicting variable linkage".to_string());
            }
            _ => {}
        }

        match &old_decl.identifier_attrs {
            IdentifierAttr::StaticAttr(InitialValue::Initial(_), _) => {
                return Err("Conflicting file scope variable definition".to_string());
            }
            IdentifierAttr::StaticAttr(init, _) => {
                initial_value = *init;
            }
            _ if matches!(initial_value, InitialValue::Initial(_))
                && matches!(
                    old_decl.identifier_attrs,
                    IdentifierAttr::StaticAttr(InitialValue::Tentative, _)
                ) =>
            {
                initial_value = InitialValue::Tentative;
            }
            _ => {}
        }
    }

    symbol_table.insert(
        var_declaration.name.clone(),
        SymbolEntry {
            sym_type: Type::Int,
            defined: true,
            identifier_attrs: IdentifierAttr::StaticAttr(initial_value, global),
        },
    );

    Ok(())
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
            typecheck_local_var_declaration(var_declaration, symbol_table)
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
        }
        Statement::Expression(expression) => {
            typecheck_expression(expression, symbol_table)?;
        }
        Statement::Null => {}
        Statement::If(expression, statement1, statement2) => {
            typecheck_expression(expression, symbol_table)?;
            typecheck_statement(statement1, symbol_table)?;
            if let Some(statement) = statement2 {
                typecheck_statement(statement, symbol_table)?;
            }
        }
        Statement::Compound(vec) => {
            typecheck_block(vec, symbol_table)?;
        }
        Statement::Break(_) => {}
        Statement::Continue(_) => {}
        Statement::While(expression, statement, _) => {
            typecheck_expression(expression, symbol_table)?;
            typecheck_statement(statement, symbol_table)?;
        }
        Statement::DoWhile(statement, expression, _) => {
            typecheck_statement(statement, symbol_table)?;
            typecheck_expression(expression, symbol_table)?;
        }
        Statement::For(for_init, expression1, expression2, statement, _) => {
            match for_init {
                ForInit::InitDeclaration(declaration) => {
                    if declaration.storage_class != None {
                        return Err("Variable declaration in for initiation can't have a storage class".to_string());
                    }
                    typecheck_local_var_declaration(declaration, symbol_table)?;
                }
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
        }
        Statement::Switch(expression, vec, statement, _) => {
            typecheck_expression(expression, symbol_table)?;
            for case in vec {
                typecheck_statement(&case.body, symbol_table)?;
            }
            if let Some(statement) = statement {
                typecheck_statement(statement, symbol_table)?;
            }
        }
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
        }
        Expression::Conditional(expression1, expression2, expression3) => {
            typecheck_expression(&expression1, symbol_table)?;
            typecheck_expression(&expression2, symbol_table)?;
            typecheck_expression(&expression3, symbol_table)?;
        }
    }

    Ok(())
}

fn typecheck_local_var_declaration(
    var_declaration: &VariableDeclaration,
    symbol_table: &mut SymbolTable,
) -> Result<(), String> {
    if var_declaration.storage_class == Some(StorageClass::Extern) {
        if var_declaration.init != None {
            return Err("Initializer on local extern variable declaration".to_string());
        }
        if symbol_table.contains_key(&var_declaration.name) {
            let old_decl = symbol_table.get(&var_declaration.name).unwrap();
            if old_decl.sym_type != Type::Int {
                return Err("Function redeclared as variable".to_string());
            }
            Ok(())
        } else {
            symbol_table.insert(
                var_declaration.name.clone(),
                SymbolEntry {
                    sym_type: Type::Int,
                    defined: true,
                    identifier_attrs: IdentifierAttr::StaticAttr(InitialValue::NoInitializer, true),
                },
            );
            Ok(())
        }
    } else if var_declaration.storage_class == Some(StorageClass::Static) {
        let initial_value = if let Some(Expression::Constant(val)) = var_declaration.init {
            InitialValue::Initial(val)
        } else if var_declaration.init == None {
            InitialValue::Initial(0)
        } else {
            return Err("Non-constant initializer on local static variable".to_string());
        };
        symbol_table.insert(
            var_declaration.name.clone(),
            SymbolEntry {
                sym_type: Type::Int,
                defined: true,
                identifier_attrs: IdentifierAttr::StaticAttr(initial_value, false),
            },
        );
        Ok(())
    } else {
        symbol_table.insert(
            var_declaration.name.clone(),
            SymbolEntry {
                sym_type: Type::Int,
                defined: true,
                identifier_attrs: IdentifierAttr::LocalAttr,
            },
        );
        if let Some(init) = &var_declaration.init {
            typecheck_expression(init, symbol_table)?
        }
        Ok(())
    }
}
