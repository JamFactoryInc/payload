use crate::util::*;
use crate::util::MatchStatus::*;
use crate::matcher::matcher::Matcher;
use std::marker::PhantomData;


pub struct Ordered<T : Matcher<K>, K, const LEN: usize> {
    cursor: usize,
    enabled: ActiveStatus,
    elements: [T; LEN],
    _phantom: PhantomData<K>,
}
impl<T : Matcher<K>, K, const LEN: usize> Matcher<K> for Ordered<T, K, LEN> {
    #[inline]
    fn matches(&mut self, next: &K) -> MatchStatus {
        if self.enabled == ActiveStatus::Off {
            return NoMatch
        }
        if LEN - 1 == self.cursor {
            self.elements[self.cursor].matches(next)
        } else {
            match self.elements[self.cursor].matches(next) {
                Match => {
                    self.cursor += 1;
                    Consumed
                }
                Consumed | GreedyMatch => {
                    Consumed
                }
                NoMatch => {
                    self.enabled = ActiveStatus::Off;
                    NoMatch
                },
                LazyMatch => {
                    LazyMatch
                }
            }
        }
    }
    #[inline] fn is_active(&self) -> &ActiveStatus { &self.enabled }
    #[inline] fn activate(&mut self) { self.enabled = ActiveStatus::On }
    #[inline] fn deactivate(&mut self) { self.enabled = ActiveStatus::Off }
    #[inline]
    fn reset(&mut self) {
        self.enabled = ActiveStatus::On;
        self.cursor = 0;
        for i in 0..LEN {
            self.elements[i].reset()
        }
    }
}