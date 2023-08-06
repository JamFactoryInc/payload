
pub(crate) enum ProductType {
    None,
    Byte,
    Vec(Box<ProductType>),
    String,
    U128,
    USize,
    I64,
    I32,
    I16,
    Option(Box<ProductType>),
    Tuple(Vec<ProductType>),
    Array(Vec<ProductType>),
    Custom(String),
}