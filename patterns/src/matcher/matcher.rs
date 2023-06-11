use crate::util::*;

pub trait Matcher<Type = Self> {
    /// Returns info regarding how well the given value matched the required parameters.
    ///
    /// Options are as follows:
    ///
    /// - [MatchStatus::NoMatch](NoMatch)
    /// This violated the pattern and this matcher's top-most parent will be disabled
    ///
    ///  - [MatchStatus::Match](Match)
    /// This completed a match and, depending on the order of patterns and when they began matching,
    /// will complete the pattern and reset
    ///
    ///  - [MatchStatus::Consumed](Consumed)
    /// The given value has been accepted but the match is not over
    ///
    ///  - [MatchStatus::LazyMatch](LazyMatch)
    /// The given value has been accepted as a valid stopping point. The top-most parent of this
    /// matcher will be marked as 'lazy' and [Matcher::matches] will be invoked only
    /// if there are no other pending options until either a match has been found that terminates
    /// this lazy matcher or until this lazy matcher returns [MatchStatus::Match]
    ///
    ///  - [MatchStatus::GreedyMatch](GreedyMatch)
    /// The given value has been accepted as a match, but the pattern may still continue. Inactive
    /// matchers will be re-activated but the greedy matcher will continue until:
    ///
    ///     - it returns [MatchStatus::NoMatch], in which case it will capture up until the last time it
    /// returned GreedyMatch
    ///
    ///     - it returns [MatchStatus::Match], in which case it will capture everything it's matched so far and
    /// all patterns will be reset (typical behaviour for Match)
    ///
    ///     - a higher-priority matcher that began before or at the same token as this returns
    /// [MatchStatus::Match]
    ///
    ///  - [MatchStatus::RetroMatch(u16)](RetroMatch)
    /// This is returned when a repeating match fails part-way through a repetition. The match is
    /// still a valid match

    fn matches(&mut self, other: &Type) -> MatchStatus;
    #[inline] fn is_active(&self) -> &ActiveStatus { &ActiveStatus::On }
    #[inline] fn activate(&mut self) { }
    #[inline] fn deactivate(&mut self) { }
    #[inline] fn reset(&mut self) { self.activate() }
    #[inline] fn get_off_your_lazy_ass(&mut self) { }
}