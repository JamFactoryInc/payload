use std::ops::AddAssign;

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