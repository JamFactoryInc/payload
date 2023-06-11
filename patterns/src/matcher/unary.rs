use crate::util::*;
use crate::matcher::matcher::Matcher;
use std::marker::PhantomData;


pub struct Unary<T : Matcher<K>, K> {
    enabled: ActiveStatus,
    element: T,
    _phantom: PhantomData<K>
}
impl<T : Matcher<K>, K> Matcher<K> for Unary<T, K> {
    #[inline] fn matches(&mut self, next: &K) -> MatchStatus {
        self.element.matches(next)
    }
    #[inline] fn is_active(&self) -> &ActiveStatus { &self.enabled }
    #[inline] fn activate(&mut self) { self.enabled = ActiveStatus::On }
    #[inline] fn deactivate(&mut self) { self.enabled = ActiveStatus::Off }
    #[inline] fn reset(&mut self) {
        self.activate();
        self.element.reset()
    }
}