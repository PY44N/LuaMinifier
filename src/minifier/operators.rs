use lua_parser::ast::{BinaryOperator, UnaryOperator};

pub fn get_binary_operator(operator: &BinaryOperator) -> String {
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

pub fn is_compressable_binary_operator(operator: &BinaryOperator) -> bool {
    *operator != BinaryOperator::And && *operator != BinaryOperator::Or
}

pub fn get_unary_operator(operator: &UnaryOperator) -> String {
    String::from(match operator {
        UnaryOperator::Minus => "-",
        UnaryOperator::Not => "not ",
        UnaryOperator::Length => "#",
        UnaryOperator::NoUnary => panic!("Got operator nounary"),
    })
}
