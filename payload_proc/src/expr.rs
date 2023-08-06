use crate::parse::ParseResult;

pub(crate) enum Expr {
    None,
    Binary(Box<Expr>, ExprOperator, Box<Expr>),
    Postfix(Box<Expr>, ExprOperator),
    Prefix(ExprOperator, Box<Expr>),
    String(String),
    Int(String),
}
impl Default for Expr {
    fn default() -> Self {
        Expr::None
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