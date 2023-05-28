use lua_parser::ast::Expression;

use crate::minifier::{
    operators::get_binary_operator, parameters::parameter_list_minification,
    statements::statement_list_minification, variables::get_variable,
};

use super::{
    fields::field_list_minification,
    functions::function_call_minification,
    methods::method_call_minification,
    operators::{get_unary_operator, is_compressable_binary_operator},
};

pub fn expression_minification(expression: &Expression) -> String {
    match expression {
        Expression::True => String::from("true"),
        Expression::False => String::from("false"),
        Expression::Nil => String::from("nil"),
        Expression::Number(num) => num.to_string(),
        Expression::String(val) => format!("\"{}\"", val),
        Expression::Dots => String::from("..."),
        Expression::Ident(val) => get_variable(val.clone()),
        Expression::AttrGet(attr, val) => {
            format!(
                "{}[{}]",
                expression_minification(attr),
                expression_minification(val)
            )
        }
        Expression::Table(field_list) => format!("{{{}}}", field_list_minification(field_list)),
        Expression::FuncCall(call) => function_call_minification(call),
        Expression::MethodCall(method) => method_call_minification(method),
        Expression::BinaryOp(operator, left, right) => {
            if is_compressable_binary_operator(operator) {
                format!(
                    "({}{}{})",
                    expression_minification(left),
                    get_binary_operator(operator),
                    expression_minification(right)
                )
            } else {
                format!(
                    "({} {} {})",
                    expression_minification(left),
                    get_binary_operator(operator),
                    expression_minification(right)
                )
            }
        }
        Expression::UnaryOp(operator, identifier) => format!(
            "{}{}",
            get_unary_operator(operator),
            expression_minification(identifier)
        ),
        Expression::Function(parameters, body) => {
            format!(
                "function({}) {}end",
                parameter_list_minification(parameters),
                statement_list_minification(body)
            )
        }
    }
}

pub fn expression_list_minification(expression_list: &Vec<Expression>) -> String {
    let mut ret = String::new();

    for (i, expr) in expression_list.iter().enumerate() {
        if i != 0 {
            ret += ",";
        }
        ret += &expression_minification(expr);
    }

    ret
}
