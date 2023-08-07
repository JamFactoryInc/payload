use crate::modifier::ModifierType::*;

const DEFAULT_ERROR: Result<ModifierType, String> = Err(r#"
Expected valid modifier name. Possible values:
 - @link: tells Payload to look for additional source text at the given path
 - @visibility: allows one path to view the available symbols in another
 - @type: defines a type as a member of a hierarchy that can be matched against
 - @bind: binds the output of a matcher to a struct field
 - @map: changes the output of a matcher with custom logic
 - @preprocess: custom logic to manipulate the input SIMD vector before it's processed
 - @postprocess: custom logic overriding Payload's default conversion of SIMD -> value
 - @naive: assumes input text is valid for the sake of performance at the cost of undefined behavior
 - @symbol: defines a symbol visible to a given scope that can be matched against
 - @error: override the default error message emitted by payload for this context
 - @ignore: marks any valid match for the given matcher to be ignored unless otherwise marked as @exact
 - @exact: disables @ignore within a block
 - @!exact: re-enables @ignore within an @exact block
"#.to_string());

pub(crate) enum ModifierType {
    Visibility,
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
    Exact,
    NotExact,
}
impl TryFrom<String> for ModifierType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, String> {
        match value.as_str() {
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
            "exact" => Ok(Exact),
            "!exact" => Ok(NotExact),
            str => {
                if str.len() == 0 {
                    return DEFAULT_ERROR
                }
                Err( format!("Did you mean {}?", match str.as_bytes()[0] {
                    b'l' => "link",
                    b'v' => "visibility",
                    b't' => "type",
                    b'b' => "bind",
                    b'm' => "map",
                    b'p' => "preprocess or postprocess",
                    b'n' => "naive",
                    b's' => "symbol",
                    b'e' => "error or exact",
                    b'i' => "ignore",
                    b'!' => "!exact",
                    _ => return DEFAULT_ERROR
                }))

            }
        }
    }
}