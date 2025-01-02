use crate::parser::*;

#[derive(Clone, Copy, PartialEq)]
enum InStatement {
    Other,
    Loop,
    Switch,
}

pub fn analyze_semantics(mut program: Program) -> Result<Program, String> {
    for function in program.functions.iter_mut() {
        if let Some(body) = &mut function.body {
            label_block(body, None)?;
        }
    }
    Ok(program)
}

fn label_block(block: &mut Block, label: Option<String>) -> Result<(), String> {
    for block_item in block {
        match block_item {
            BlockItem::D(_) => {}
            BlockItem::S(statement) => {
                label_statement(statement, label.clone(), InStatement::Other)?;
            }
        }
    }
    Ok(())
}
fn label_statement(
    statement: &mut Statement,
    label: Option<String>,
    in_statement: InStatement,
) -> Result<(), String> {
    match statement {
        Statement::Compound(block) => {
            label_block(block, label)?;
        }
        Statement::If(_, if_block, else_block) => {
            label_statement(if_block, label.clone(), in_statement)?;
            if let Some(else_body) = else_block {
                label_statement(else_body, label, in_statement)?;
            }
        }
        Statement::While(_, body, label_opt) => {
            let new_label = make_label_name("while");
            label_statement(body, Some(new_label.clone()), InStatement::Loop)?;
            *label_opt = Some(new_label);
        }
        Statement::DoWhile(body, _, label_opt) => {
            let new_label = make_label_name("do_while");
            label_statement(body, Some(new_label.clone()), InStatement::Loop)?;
            *label_opt = Some(new_label);
        }
        Statement::For(_, _, _, body, label_opt) => {
            let new_label = make_label_name("for");
            label_statement(body, Some(new_label.clone()), InStatement::Loop)?;
            *label_opt = Some(new_label);
        }
        Statement::Break(label_opt) => {
            if label.is_none() {
                return Err("Break statement outside of loop".to_string());
            }
            *label_opt = label;
        }
        Statement::Continue(label_opt) => {
            if label.is_none() || in_statement == InStatement::Switch {
                return Err("Continue statement outside of loop".to_string());
            }
            *label_opt = label;
        }
        Statement::Return(_) => {}
        Statement::Expression(_) => {}
        Statement::Null => {}
        Statement::Switch(_, cases, default, label_opt) => {
            let new_label = make_label_name("switch");
            for case in cases {
                label_statement(&mut case.body, Some(new_label.clone()), InStatement::Switch)?;
            }
            if let Some(default) = default {
                label_statement(&mut *default, Some(new_label.clone()), InStatement::Switch)?;
            }
            *label_opt = Some(new_label);
        }
    }

    Ok(())
}

static mut LABEL_COUNTER: i64 = -1;

fn make_label_name(prefix: &str) -> String {
    format!("label_{}.{}", prefix, unsafe {
        LABEL_COUNTER += 1;
        LABEL_COUNTER
    })
}
