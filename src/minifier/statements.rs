use lua_parser::ast::Statement;

use crate::minifier::{expressions::minify_expression_list, parameters::minify_string_list};

use super::{
    expressions::minify_expression,
    for_loops::{minify_generic_for, minify_numeric_for},
    functions::minify_function_definition,
    if_statements::minify_if,
    methods::minify_method_definition,
};

pub fn minify_statement(statement: &Statement) -> String {
    match statement {
        Statement::Assign(left, right) => {
            format!(
                "{}={}",
                minify_expression_list(left),
                minify_expression_list(right)
            )
        }
        Statement::LocalAssign(parameters, expression_list) => {
            format!(
                "local {}={}",
                minify_string_list(parameters),
                minify_expression_list(expression_list)
            )
        }
        Statement::FuncCall(expr) => minify_expression(expr),
        Statement::MethodCall(method) => minify_expression(method),
        Statement::DoBlock(body) => format!("do {}end", minify_statement_list(body)),
        Statement::While(check, body) => format!(
            "while {} do {}end",
            minify_expression(check),
            minify_statement_list(body)
        ),
        Statement::Repeat(case, body) => format!(
            "repeat {}until {}",
            minify_statement_list(body),
            minify_expression(case)
        ),
        Statement::If(if_statement) => minify_if(if_statement),
        Statement::NumberFor(for_loop) => minify_numeric_for(for_loop),
        Statement::GenericFor(for_loop) => minify_generic_for(for_loop),
        Statement::FuncDef(def) => minify_function_definition(def),
        Statement::MethodDef(method) => minify_method_definition(method),
        Statement::Return(args) => {
            let exprs = minify_expression_list(args);
            if exprs == "" {
                String::from("return")
            } else {
                format!("return {}", exprs)
            }
        }
        Statement::Break => String::from("break"),
    }
}

pub fn minify_statement_list(statement_list: &Vec<Statement>) -> String {
    let mut ret = String::new();

    for statement in statement_list {
        let state = minify_statement(&statement) + "; ";
        ret += &state;
    }

    ret
}
