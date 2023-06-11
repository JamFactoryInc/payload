use crate::util::*;
use crate::matcher::matcher::Matcher;
use crate::matcher::unary::*;
use crate::matcher::ordered::*;
use crate::range::*;

pub enum SimpleMatcher<'a, T : Matcher<K>, K> {
    Unary(&'a mut Unary<T, K>),
    Binary(&'a mut Ordered<T, K, 2>),
    Ternary(&'a mut Ordered<T, K, 3>),
    Quartary(&'a mut Ordered<T, K, 4>),
    Optional(&'a mut SimpleMatcher<'a, T, K>),
    Repeated(&'a mut SimpleMatcher<'a, T, K>, Range),
    Lazy(&'a mut SimpleMatcher<'a, T, K>, LazyRange),
}
impl<'a, T : Matcher<K>, K> Matcher<K> for SimpleMatcher<'a, T, K> {
    fn activate(&mut self) {
        match self {
            Self::Unary(matcher) => matcher.activate(),
            Self::Binary(matcher) => matcher.activate(),
            Self::Ternary(matcher) => matcher.activate(),
            Self::Quartary(matcher) => matcher.activate(),
            Self::Optional(matcher) => matcher.activate(),
            Self::Repeated(matcher, ..) => matcher.activate(),
            Self::Lazy(matcher, ..) => matcher.activate(),
        }
    }
    fn deactivate(&mut self) {
        match self {
            Self::Unary(matcher) => matcher.deactivate(),
            Self::Binary(matcher) => matcher.deactivate(),
            Self::Ternary(matcher) => matcher.deactivate(),
            Self::Quartary(matcher) => matcher.deactivate(),
            Self::Optional(matcher) => matcher.deactivate(),
            Self::Repeated(matcher, ..) => matcher.deactivate(),
            Self::Lazy(matcher, ..) => matcher.deactivate(),
        }
    }
    fn matches(&mut self, next: &K) -> MatchStatus {
        match self {
            Self::Unary(matcher) => matcher.matches(next),
            Self::Binary(matcher) => matcher.matches(next),
            Self::Ternary(matcher) => matcher.matches(next),
            Self::Quartary(matcher) => matcher.matches(next),
            Self::Optional(matcher) => matcher.matches(next),
            Self::Repeated(matcher, range) => {

                matcher.matches(next)
            },
            Self::Lazy(matcher, range) => {
                matcher.matches(next)
            },
        }
    }
    fn reset(&mut self) {
        match self {
            Self::Unary(matcher) => matcher.reset(),
            Self::Binary(matcher) => matcher.reset(),
            Self::Ternary(matcher) => matcher.reset(),
            Self::Quartary(matcher) => matcher.reset(),
            Self::Optional(matcher) => matcher.reset(),
            Self::Repeated(matcher, ..) => matcher.reset(),
            Self::Lazy(matcher, ..) => matcher.reset(),
        }
    }
    fn is_active(&self) -> &ActiveStatus {
        match self {
            Self::Unary(matcher) => matcher.is_active(),
            Self::Binary(matcher) => matcher.is_active(),
            Self::Ternary(matcher) => matcher.is_active(),
            Self::Quartary(matcher) => matcher.is_active(),
            Self::Optional(matcher) => matcher.is_active(),
            Self::Repeated(matcher, ..) => matcher.is_active(),
            Self::Lazy(matcher, ..) => matcher.is_active(),
        }
    }
}