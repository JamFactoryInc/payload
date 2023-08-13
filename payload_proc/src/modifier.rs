use std::fmt::{Debug, Display, Formatter};
use crate::describe::Describe;
use crate::modifier::ModifierType::*;
use std::fmt::Write as _;

#[derive(Clone)]
pub(crate) enum ModifierType {
    Visibility,
    Scope,
    Link,
    Type,
    Bind,
    Map,
    PreProcess,
    PostProcess,
    Naive,
    Symbol,
    Error,
    Ignore,
    Strict,
    Loose,
    Nil,
}
impl ModifierType {
     fn as_string_repr(&self) -> String {
        match self {
            Visibility => "visibility",
            Scope => "scope",
            Link => "link",
            Type => "type",
            Bind => "bind",
            Map => "map",
            PreProcess => "preprocess",
            PostProcess => "postprocess",
            Naive => "naive",
            Symbol => "symbol",
            Error => "error",
            Ignore => "ignore",
            Strict => "strict",
            Loose => "loose",
            Nil => "<nil>",
        }.to_string()
    }

    fn get_default_error() -> String {
        let mut out = "Expected valid modifier name. Possible values:".to_string();
        let patterns: Vec<Self> = Nil.into();
        for pattern in patterns {
            writeln!(&mut out, " - {:?}: {}", pattern, pattern.describe()).unwrap()
        }
        out
    }
}
impl TryFrom<String> for ModifierType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, String> {
        println!("attempting to parse {:?} as a Modifier", value);
        match value.as_str() {
            "scope" => Ok(Scope),
            "link" => Ok(Link),
            "visibility" => Ok(Visibility),
            "type" => Ok(Type),
            "bind" => Ok(Bind),
            "map" => Ok(Map),
            "preprocess" => Ok(PreProcess),
            "postprocess" => Ok(PostProcess),
            "naive" => Ok(Naive),
            "symbol" => Ok(Symbol),
            "error" => Ok(Error),
            "ignore" => Ok(Ignore),
            "strict" => Ok(Strict),
            "loose" => Ok(Loose),
            str => {
                if str.is_empty() {
                    return Err(ModifierType::get_default_error())
                }
                Err( format!("Did you mean {}?", match str.as_bytes()[0] {
                    b'l' => "link or loose",
                    b'v' => "visibility",
                    b't' => "type",
                    b'b' => "bind",
                    b'm' => "map",
                    b'p' => "preprocess or postprocess",
                    b'n' => "naive",
                    b's' => "symbol or strict",
                    b'e' => "error",
                    b'i' => "ignore",
                    _ => return Err(ModifierType::get_default_error())
                }))

            }
        }
    }
}
impl Into<Vec<Self>> for ModifierType {
    fn into(self) -> Vec<Self> {
        vec![
            Visibility,
            Scope,
            Link,
            Type,
            Bind,
            Map,
            PreProcess,
            PostProcess,
            Naive,
            Symbol,
            Error,
            Ignore,
            Strict,
            Loose,
            Nil,
        ]
    }
}
impl Debug for ModifierType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.as_string_repr())
    }
}
impl Display for ModifierType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.as_string_repr())
    }
}
impl Describe for ModifierType {
    fn describe(&self) -> &'static str {
        match self {
            Link => "tells Payload to look for additional source text at the given path",
            Scope => "indicates a scope to be set after the affected matcher executes",
            Visibility => "allows one path to view the available symbols in another",
            Type => "defines a type as a member of a hierarchy that can be matched against",
            Bind => "binds the output of a matcher to a struct field",
            Map => "changes the output of a matcher with custom logic",
            PreProcess => "custom logic to manipulate the input SIMD vector before it's processed",
            PostProcess => "custom logic overriding Payload's default conversion of SIMD -> value",
            Naive => "assumes input text is valid for the sake of performance at the cost of undefined behavior",
            Symbol => "defines a symbol visible to a given scope that can be matched against",
            Error => "override the default error message emitted by payload for this context",
            Ignore => "marks any valid match for the given matcher to be ignored unless otherwise marked as @exact",
            Loose => "disables @ignore within a block",
            Strict => "re-enables @ignore within an @exact block",
            Nil => "",
        }
    }
}