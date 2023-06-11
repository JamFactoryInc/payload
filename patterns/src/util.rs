pub enum MatchStatus {
    NoMatch,
    Match,
    Consumed,
    LazyMatch,
    GreedyMatch,
}
#[derive(PartialEq, Clone)]
#[repr(u8)]
pub enum ActiveStatus {
    Off,
    Lazy,
    On,
}