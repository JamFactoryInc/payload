use crate::util::*;
use crate::matcher::matcher::Matcher;
use crate::range::{Range, Ranged};
use crate::matcher::simple::*;
use crate::util::MatchStatus;
use crate::util::MatchStatus::*;

pub struct RepeatedMatcher<'a, T: Matcher<K>, K> {
    range: Range,
    cursor: u16,
    enabled: ActiveStatus,
    matcher: SimpleMatcher<'a, T, K>,
}
impl<'a, T : Matcher<K>, K> RepeatedMatcher<'a, T, K> {
    #[inline]
    fn match_on(cursor : &mut u16, match_status : MatchStatus, return_if_match: MatchStatus) -> MatchStatus {
        match match_status {
            Match => {
                *cursor += 1;
                return_if_match
            },
            val @ _ => val,
        }
    }
}
impl<'a, T: Matcher<K>, K> Matcher<K> for RepeatedMatcher<'a, T, K> {
    #[inline] fn matches(&mut self, next: &K) -> MatchStatus {
        match self.range.check(self.cursor as usize) {
            NoMatch => {
                panic!()
            },
            Consumed => {
                Self::match_on(&mut self.cursor, self.matcher.matches(next), Consumed)
            },
            LazyMatch => {
                Self::match_on(&mut self.cursor, self.matcher.matches(next), LazyMatch)
            },
            Match => {
                self.matcher.matches(next)
            },
            GreedyMatch => {
                Self::match_on(&mut self.cursor, self.matcher.matches(next), GreedyMatch)
            }
        }
    }
    #[inline] fn is_active(&self) -> &ActiveStatus { &self.enabled }
    #[inline] fn activate(&mut self) { self.enabled = ActiveStatus::On }
    #[inline] fn deactivate(&mut self) { self.enabled = ActiveStatus::Off }
    #[inline] fn reset(&mut self) {
        self.activate();
        self.cursor = 0;
        self.matcher.reset();
    }
}