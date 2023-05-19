use lua_parser::ast::Expression;

use crate::minifier::{
    operators::get_binary_operator, parameters::minify_parameter_list,
    statements::minify_statement_list,
};

use super::{
    fields::minify_field_list,
    functions::minify_function_call,
    methods::minify_method_call,
    operators::{get_unary_operator, is_compressable_binary_operator},
};

pub fn minify_expression(expression: &Expression) -> String {
    match expression {
        Expression::True => String::from("true"),
        Expression::False => String::from("false"),
        Expression::Nil => String::from("nil"),
        Expression::Number(num) => num.to_string(),
        Expression::String(val) => format!("\"{}\"", val),
        Expression::Dots => String::from("..."),
        Expression::Ident(val) => val.clone(),
        Expression::AttrGet(attr, val) => {
            format!("{}[{}]", minify_expression(attr), minify_expression(val))
        }
        Expression::Table(field_list) => format!("{{{}}}", minify_field_list(field_list)),
        Expression::FuncCall(call) => minify_function_call(call),
        Expression::MethodCall(method) => minify_method_call(method),
        Expression::BinaryOp(operator, left, right) => {
            if is_compressable_binary_operator(operator) {
                format!(
                    "({}{}{})",
                    minify_expression(left),
                    get_binary_operator(operator),
                    minify_expression(right)
                )
            } else {
                format!(
                    "({} {} {})",
                    minify_expression(left),
                    get_binary_operator(operator),
                    minify_expression(right)
                )
            }
        }
        Expression::UnaryOp(operator, identifier) => format!(
            "{}{}",
            get_unary_operator(operator),
            minify_expression(identifier)
        ),
        Expression::Function(parameters, body) => {
            format!(
                "function({}) {}end",
                minify_parameter_list(parameters),
                minify_statement_list(body)
            )
        }
    }
}

pub fn minify_expression_list(expression_list: &Vec<Expression>) -> String {
    let mut ret = String::new();

    for (i, expr) in expression_list.iter().enumerate() {
        if i != 0 {
            ret += ",";
        }
        ret += &minify_expression(expr);
    }

    ret
}
