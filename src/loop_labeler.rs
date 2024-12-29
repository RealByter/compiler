use crate::parser::*;

pub fn label_loops(mut program: Program) -> Result<Program, String> {
    label_block(&mut program.function.body, None)?;
    Ok(program)
}

fn label_block(block: &mut Block, label: Option<String>) -> Result<(), String> {
    for block_item in block {
        match block_item {
            BlockItem::D(_) => {}
            BlockItem::S(statement) => {
                label_statement(statement, label.clone())?;
            }
        }
    }
    Ok(())
}
fn label_statement(statement: &mut Statement, label: Option<String>) -> Result<(), String> {
    match statement {
        Statement::Compound(block) => {
            label_block(block, label)?;
        }
        Statement::If(_, if_block, else_block) => {
            label_statement(if_block, label.clone())?;
            if let Some(else_body) = else_block {
                label_statement(else_body, label)?;
            }
        }
        Statement::While(_, body, label_opt) => {
            let new_label = make_label_name("while");
            label_statement(body, Some(new_label.clone()))?;
            *label_opt = Some(new_label);
        }
        Statement::DoWhile(body, _, label_opt) => {
            let new_label = make_label_name("do_while");
            label_statement(body, Some(new_label.clone()))?;
            *label_opt = Some(new_label);
        }
        Statement::For(_, _, _, body, label_opt) => {
            let new_label = make_label_name("for");
            label_statement(body, Some(new_label.clone()))?;
            *label_opt = Some(new_label);
        }
        Statement::Break(label_opt) => {
            if label.is_none() {
                return Err("Break statement outside of loop".to_string());
            }
            *label_opt = label;
        }
        Statement::Continue(label_opt) => {
            if label.is_none() {
                return Err("Continue statement outside of loop".to_string());
            }
            *label_opt = label;
        }
        Statement::Return(_) => {}
        Statement::Expression(_) => {}
        Statement::Null => {}
        Statement::Switch(expression, vec, statement) => todo!(),
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
