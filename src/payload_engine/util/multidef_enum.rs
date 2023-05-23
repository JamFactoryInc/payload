use std::ops::AddAssign;

/// Mdef is a way of declaring enum values across multiple files without much overhead, as
/// defined constants are just usize aliases
///
/// This is useful if default enumerations are required in addition to later-implemented ones
pub struct MultidefEnum<'a, T>
    where T : From<usize>
        + Into<usize>
        + AddAssign<usize>
        + Copy {
    names : Vec<&'a str>,
    index : T
}

impl<'a, T> MultidefEnum<'a, T>
    where T : From<usize>
        + Into<usize>
        + AddAssign<usize>
        + Copy {

    pub fn new<'b>() -> MultidefEnum<'b, T> {
        return MultidefEnum {
            names : Vec::new(),
            index : T::from(0)
        }
    }

    pub fn nameof(&self, index : T) -> &str {
        self.names[T::into(index)]
    }

    pub fn create_type(&mut self, name : &'a str) -> T {
        self.names.push(name);

        let ret = self.index.clone();

        self.index += 1;

        return ret
    }
}