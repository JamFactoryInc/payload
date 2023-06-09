
pub mod payload_engine;
pub mod interface;
mod test;

pub mod payload {

}

#[cfg(test)]
mod tests {
    use std::mem::size_of;
    use regex_proc::regex;
    use patterns::patterns;
    use test;

    #[test]
    fn sanity_check() {
        println!("{}", size_of::<patterns::Range>());
        println!("{}", size_of::<patterns::LazyRange>());
        //regex!(abc);
    }
}
