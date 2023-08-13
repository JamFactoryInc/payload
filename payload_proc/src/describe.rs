
pub(crate) trait Describe {
    fn describe(&self) -> &'static str;
}