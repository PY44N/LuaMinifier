use lua_parser::ast::{
    BinaryOperator, Expression, Field, FunctionCall, FunctionDefinition, GenericFor, IfThenElse,
    NumberFor, ParameterList, Statement, UnaryOperator,
};

fn get_binary_operator(operator: &BinaryOperator) -> String {
    String::from(match operator {
        BinaryOperator::Add => "+",
        BinaryOperator::Sub => "-",
        BinaryOperator::Mul => "*",
        BinaryOperator::Div => "/",
        BinaryOperator::Mod => "%",
        BinaryOperator::Pow => "^",
        BinaryOperator::Concat => "..",
        BinaryOperator::Eq => "==",
        BinaryOperator::LT => "<",
        BinaryOperator::LE => "<=",
        BinaryOperator::NE => "~=",
        BinaryOperator::GT => ">",
        BinaryOperator::GE => ">=",
        BinaryOperator::And => "and",
        BinaryOperator::Or => "or",
        BinaryOperator::NoBinary => panic!("Got operator nobinary"),
    })
}

fn get_unary_operator(operator: &UnaryOperator) -> String {
    String::from(match operator {
        UnaryOperator::Minus => "-",
        UnaryOperator::Not => "not ",
        UnaryOperator::Length => "#",
        UnaryOperator::NoUnary => panic!("Got operator nounary"),
    })
}

fn minify_string_list(parameters: &Vec<String>) -> String {
    let mut ret = String::new();

    for (i, param) in parameters.iter().enumerate() {
        if i != 0 {
            ret += ",";
        }
        ret += param;
    }

    ret
}

fn minify_field(field: &Field) -> String {
    if let Some(key) = &field.key {
        String::from(format!(
            "[{}] = {}",
            minify_expression(key),
            minify_expression(&field.val)
        ))
    } else {
        String::from(minify_expression(&field.val))
    }
}

fn minify_field_list(fields: &Vec<Field>) -> String {
    let mut ret = String::new();

    for (i, field) in fields.iter().enumerate() {
        if i != 0 {
            ret += ",";
        }
        ret += &minify_field(field);
    }

    ret
}

fn minify_parameter_list(list: &ParameterList) -> String {
    let mut ret = minify_string_list(&list.names);

    if list.vargs {
        if list.names.len() != 0 {
            ret += ",";
        }
        ret += "...";
    }

    ret
}

fn minify_function_call(call: &FunctionCall) -> String {
    format!(
        "{}({})",
        minify_expression(&call.func),
        minify_expression_list(&call.args)
    )
}

fn minify_function_definition(def: &FunctionDefinition) -> String {
    format!(
        "{} = {}",
        minify_expression_list(&def.name),
        minify_expression_list(&def.body)
    )
}

fn minify_generic_for(for_loop: &GenericFor) -> String {
    format!(
        "for {} in {} do {}end",
        minify_string_list(&for_loop.names),
        minify_expression_list(&for_loop.exprs),
        minify_statement_list(&for_loop.stmts)
    )
}

fn minify_numeric_for(for_loop: &NumberFor) -> String {
    // TODO: Some of these arguments are optional
    format!(
        "for {}={},{},{} do {}end",
        for_loop.name,
        minify_expression(&for_loop.init),
        minify_expression(&for_loop.limit),
        minify_expression(&for_loop.step),
        minify_statement_list(&for_loop.stmts)
    )
}

fn minify_if(if_statement: &IfThenElse) -> String {
    format!(
        "if {} then {}else {}end",
        minify_expression(&if_statement.condition),
        minify_statement_list(&if_statement.then),
        minify_statement_list(&if_statement.els),
    )
}

fn minify_expression(expression: &Expression) -> String {
    match expression {
        Expression::True => todo!(),
        Expression::False => todo!(),
        Expression::Nil => todo!(),
        Expression::Number(num) => num.to_string(),
        Expression::String(val) => format!("\"{}\"", val),
        Expression::Dots => String::from("..."),
        Expression::Ident(val) => val.clone(),
        Expression::AttrGet(attr, val) => {
            format!("{}[{}]", minify_expression(attr), minify_expression(val))
        }
        Expression::Table(field_list) => format!("{{{}}}", minify_field_list(field_list)),
        Expression::FuncCall(call) => minify_function_call(call),
        Expression::MethodCall(_) => todo!(),
        Expression::BinaryOp(operator, left, right) => format!(
            "{}{}{}",
            minify_expression(left),
            get_binary_operator(operator),
            minify_expression(right)
        ),
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

fn minify_expression_list(expression_list: &Vec<Expression>) -> String {
    let mut ret = String::new();

    for (i, expr) in expression_list.iter().enumerate() {
        if i != 0 {
            ret += ",";
        }
        ret += &minify_expression(expr);
    }

    ret
}

fn minify_statement(statement: &Statement) -> String {
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
        Statement::MethodCall(_) => todo!(),
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
        Statement::MethodDef(_) => todo!(),
        Statement::Return(_) => todo!(),
        Statement::Break => todo!(),
    }
}

fn minify_statement_list(statement_list: &Vec<Statement>) -> String {
    let mut ret = String::new();

    for statement in statement_list {
        let state = minify_statement(&statement) + "; ";
        println!("{}", state);
        ret += &state;
    }

    ret
}

pub struct Minifier {
    ast: Vec<Statement>,
}

impl Minifier {
    pub fn new(ast: Vec<Statement>) -> Self {
        Self { ast }
    }

    pub fn minify(&mut self) -> String {
        minify_statement_list(&self.ast)
    }
}
