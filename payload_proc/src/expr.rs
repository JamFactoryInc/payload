use crate::parse::ParseResult;
use crate::variable::Variable;

pub(crate) enum Expr {
    None,
    Binary(Box<Expr>, ExprOperator, Box<Expr>),
    Postfix(Box<Expr>, ExprOperator),
    Prefix(ExprOperator, Box<Expr>),
    String(String),
    Int(String),
    Variable(Variable),
}
impl Default for Expr {
    fn default() -> Self {
        Expr::None
    }
}
impl TryFrom<String> for Expr {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        todo!()
    }
}

pub(crate) enum ExprOperator {
    Assign,
}

pub(crate) struct ExprParser {

}
impl ExprParser {
    pub(crate) fn parse(&mut self, char: u8) -> ParseResult {
        todo!()
    }
}

impl Default for ExprParser {
    fn default() -> Self {
        ExprParser {}
    }
}