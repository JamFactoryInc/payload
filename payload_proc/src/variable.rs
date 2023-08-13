use crate::variable::Variable::{Match, Scope, Type};

#[derive(Debug)]
pub(crate) enum Variable {
    Match,
    Type,
    Scope,
}
impl TryFrom<String> for Variable {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "match" => Ok(Match),
            "type" => Ok(Type),
            "scope" => Ok(Scope),
            _ => Err(format!("Unknown variable {value:?}"))
        }
    }
}