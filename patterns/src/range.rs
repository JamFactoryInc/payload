use crate::util::MatchStatus::*;
use crate::util::*;

pub trait Ranged {
    /// Returns info based on the quality of the match
    ///
    /// - **Consumed**: Illegal end-point but keep going
    ///
    /// - **GreedyMatch**: Legal end-point but keep going
    ///
    /// - **LazyMatch**: Legal end-point and recommended stopping-point, but continue if necessary
    ///
    /// - **Match**: Last legal end-point. Stop
    fn check(&self, num: usize) -> MatchStatus;
}

/// Indicates the number of repeats a pattern can match
///
/// This is the greedy variant, matching as many as possible
///
/// Uses u16 values to fit in a single word on 64-bit systems and parallel rust's regex constraints
pub enum Range {
    /// Zero or more times. Same as `*` or `{0,}`
    FromZero,
    /// One or more times. Same as `+` or `{1,}`
    FromOne,
    /// Zero or one times. Same as `?` or `{,1}`
    Optional,
    /// <x> or more times (inclusive). Same as `{<x>,}`
    From(u16),
    /// Zero to <x> times (inclusive). Same as `{,<x>}`
    To(u16),
    /// Exactly <x> times. Same as `{<x>}`
    Times(u16),
    /// Between <x> and <y> times (inclusive). Same as `{<x>,<y>}`
    Range(u16, u16),
}
impl Ranged for Range {
    fn check(&self, num : usize) -> MatchStatus {
        match self {
            Self::FromZero => GreedyMatch,
            Self::Optional => if num == 0 {
                GreedyMatch
            } else {
                Match
            },
            Self::FromOne => if num == 0 {
                Consumed
            } else {
                GreedyMatch
            },
            Self::From(val) => if num < *val as usize {
                Consumed
            } else {
                GreedyMatch
            },
            Self::To(val) => if num < *val as usize {
                GreedyMatch
            } else {
                Match
            },
            Self::Times(val) => if num == *val as usize {
                Match
            } else {
                Consumed
            },
            Self::Range(from, to) => if num < *from as usize {
                Consumed
            } else if num < *to as usize {
                GreedyMatch
            } else {
                Match
            },
        }
    }
}
/// Indicates the number of repeats a pattern can match
///
/// This is the lazy variant, matching as few as possible
///
/// As this is a newtype of Range, the contents of LazyRange are identical:
///
/// - [Range::FromZero](FromZero)
///
/// - [Range::FromOne](FromOne)
///
/// - [Range::From](From)
///
/// - [Range::To](To)
///
/// - [Range::Times](Times)
///
/// - [Range::Range](Range)
///
/// Uses u16 values to fit in a single word on 64-bit systems and parallel rust's regex constraints
pub enum LazyRange {
    /// Zero or more times. Same as `*` or `{0,}`
    FromZero,
    /// One or more times. Same as `+` or `{1,}`
    FromOne,
    /// Zero or one times. Same as `?` or `{,1}`
    Optional,
    /// <x> or more times (inclusive). Same as `{<x>,}`
    From(u16),
    /// Zero to <x> times (inclusive). Same as `{,<x>}`
    To(u16),
    /// Exactly <x> times. Same as `{<x>}`
    Times(u16),
    /// Between <x> and <y> times (inclusive). Same as `{<x>,<y>}`
    Range(u16, u16),
}
impl Ranged for LazyRange {
    fn check(&self, num: usize) -> MatchStatus {
        match self {
            Self::FromZero => LazyMatch,
            Self::Optional => if num == 0 {
                LazyMatch
            } else {
                Match
            },
            Self::FromOne => if num == 0 {
                Consumed
            } else {
                Match
            },
            Self::From(val) => if num < *val as usize {
                Consumed
            } else {
                LazyMatch
            },
            Self::To(val) => if num < *val as usize {
                LazyMatch
            } else {
                Match
            },
            Self::Times(val) => if num < *val as usize {
                Consumed
            } else {
                Match
            },
            Self::Range(from, to) => if num < *from as usize {
                Consumed
            } else if num < *to as usize {
                LazyMatch
            } else {
                Match
            },
        }
    }
}
