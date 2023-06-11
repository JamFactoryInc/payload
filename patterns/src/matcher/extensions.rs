use crate::util::*;
use crate::matcher::matcher::Matcher;
use crate::matcher::ordered::*;
use std::marker::PhantomData;

pub enum BinaryCompound<'a, T : Matcher<K>, K, MA : Matcher<T>, MB : Matcher<T>> {
    MatcherSetA(&'a mut MA),
    MatcherSetB(&'a mut MB),
    #[deprecated(note = "Do not use this; Only present for type enforcement")]
    _Phantom(PhantomData<T>),
    #[deprecated(note = "Do not use this; Only present for type enforcement")]
    _Phantom2(PhantomData<K>),
}
impl<'a, T : Matcher<K>, K, MA : Matcher<T>, MB : Matcher<T>> Matcher<T> for BinaryCompound<'a, T, K, MA, MB> {
    fn activate(&mut self) {
        match self {
            Self::MatcherSetA(matcher) => matcher.activate(),
            Self::MatcherSetB(matcher) => matcher.activate(),
            _ => panic!()
        }
    }
    fn deactivate(&mut self) {
        match self {
            Self::MatcherSetA(matcher) => matcher.deactivate(),
            Self::MatcherSetB(matcher) => matcher.deactivate(),
            _ => panic!()
        }
    }
    fn matches(&mut self, next: &T) -> MatchStatus {
        match self {
            Self::MatcherSetA(matcher) => matcher.matches(next),
            Self::MatcherSetB(matcher) => matcher.matches(next),
            _ => panic!()
        }
    }
    fn reset(&mut self) {
        match self {
            Self::MatcherSetA(matcher) => matcher.reset(),
            Self::MatcherSetB(matcher) => matcher.reset(),
            _ => panic!()
        }
    }
    fn is_active(&self) -> &ActiveStatus {
        match self {
            Self::MatcherSetA(matcher) => matcher.is_active(),
            Self::MatcherSetB(matcher) => matcher.is_active(),
            _ => panic!()
        }
    }
}

pub enum TernaryCompound<'a, T : Matcher<K>, K, MA : Matcher<T>, MB : Matcher<T>, MC : Matcher<T>> {
    MatcherSetA(&'a mut MA),
    MatcherSetB(&'a mut MB),
    MatcherSetC(&'a mut MC),
    #[deprecated(note = "Do not use this; Only present for type enforcement")]
    _Phantom(PhantomData<T>),
    #[deprecated(note = "Do not use this; Only present for type enforcement")]
    _Phantom2(PhantomData<K>)
}
impl<'a, T : Matcher<K>, K, MA : Matcher<T>, MB : Matcher<T>, MC : Matcher<T>> Matcher<T>
for TernaryCompound<'a, T, K, MA, MB, MC> {
    fn activate(&mut self) {
        match self {
            Self::MatcherSetA(matcher) => matcher.activate(),
            Self::MatcherSetB(matcher) => matcher.activate(),
            Self::MatcherSetC(matcher) => matcher.activate(),
            _ => panic!()
        }
    }
    fn deactivate(&mut self) {
        match self {
            Self::MatcherSetA(matcher) => matcher.deactivate(),
            Self::MatcherSetB(matcher) => matcher.deactivate(),
            Self::MatcherSetC(matcher) => matcher.deactivate(),
            _ => panic!()
        }
    }
    fn matches(&mut self, next: &T) -> MatchStatus {
        match self {
            Self::MatcherSetA(matcher) => matcher.matches(next),
            Self::MatcherSetB(matcher) => matcher.matches(next),
            Self::MatcherSetC(matcher) => matcher.matches(next),
            _ => panic!()
        }
    }
    fn reset(&mut self) {
        match self {
            Self::MatcherSetA(matcher) => matcher.reset(),
            Self::MatcherSetB(matcher) => matcher.reset(),
            Self::MatcherSetC(matcher) => matcher.reset(),
            _ => panic!()
        }
    }
    fn is_active(&self) -> &ActiveStatus {
        match self {
            Self::MatcherSetA(matcher) => matcher.is_active(),
            Self::MatcherSetB(matcher) => matcher.is_active(),
            Self::MatcherSetC(matcher) => matcher.is_active(),
            _ => panic!()
        }
    }
}

pub type DynamicExtension<'a, T, K, M> = BinaryCompound<'a, T, K, M, &'a dyn Matcher<T>>;
pub type DynamicDoubleExtension<'a, T, K, M> = TernaryCompound<'a, T, K, M, &'a dyn Matcher<T>, &'a dyn Matcher<T>>;
pub type NOrderedExtension<'a, T, K, M, const LEN: usize> = BinaryCompound<'a, T, K, M, &'a mut Ordered<T, K, LEN>>;
pub type NOrderedDoubleExtension<'a, T, K, M, const LEN_A: usize, const LEN_B: usize>
= TernaryCompound<'a, T, K, M, &'a mut Ordered<T, K, LEN_A>, &'a mut Ordered<T, K, LEN_B>>;