use lua_parser::ast::Statement;

use crate::minifier::{
    expressions::expression_list_minification, parameters::string_list_minification,
};

use super::{
    expressions::expression_minification,
    for_loops::{generic_for_minification, numeric_for_minification},
    functions::function_definition_minification,
    if_statements::if_minification,
    methods::method_definition_minification,
};

pub fn statement_minification(statement: &Statement) -> String {
    match statement {
        Statement::Assign(left, right) => {
            format!(
                "{}={}",
                expression_list_minification(left),
                expression_list_minification(right)
            )
        }
        Statement::LocalAssign(parameters, expression_list) => {
            format!(
                "local {}={}",
                string_list_minification(parameters),
                expression_list_minification(expression_list)
            )
        }
        Statement::FuncCall(expr) => expression_minification(expr),
        Statement::MethodCall(method) => expression_minification(method),
        Statement::DoBlock(body) => format!("do {}end", statement_list_minification(body)),
        Statement::While(check, body) => format!(
            "while {} do {}end",
            expression_minification(check),
            statement_list_minification(body)
        ),
        Statement::Repeat(case, body) => format!(
            "repeat {}until {}",
            statement_list_minification(body),
            expression_minification(case)
        ),
        Statement::If(if_statement) => if_minification(if_statement),
        Statement::NumberFor(for_loop) => numeric_for_minification(for_loop),
        Statement::GenericFor(for_loop) => generic_for_minification(for_loop),
        Statement::FuncDef(def) => function_definition_minification(def),
        Statement::MethodDef(method) => method_definition_minification(method),
        Statement::Return(args) => {
            let exprs = expression_list_minification(args);
            if exprs == "" {
                String::from("return")
            } else {
                format!("return {}", exprs)
            }
        }
        Statement::Break => String::from("break"),
    }
}

pub fn statement_list_minification(statement_list: &Vec<Statement>) -> String {
    let mut ret = String::new();

    for statement in statement_list {
        let state = statement_minification(&statement) + ";";
        ret += &state;
    }

    ret
}
