pub mod patterns {
    use crate::patterns::MatchStatus::{Consumed, Match, NoMatch, RetroMatch};

    /// Indicates the number of repeats a pattern can match
    ///
    /// This is the lazy variant, matching as few as possible
    ///
    /// Uses u16 values to fit in a single word on 64-bit systems
    pub enum LazyRange {
        /// Zero or more times. Same as `*` or `{0,}`
        FromZero,
        /// One or more times. Same as `+` or `{1,}`
        FromOne,
        /// <x> or more times (inclusive). Same as `{<x>,}`
        From(u16),
        /// Zero to <x> times (inclusive). Same as `{,<x>}`
        To(u16),
        /// Between <x> and <y> times (inclusive). Same as `{<x>,<y>}`
        Range(u8, u8),
    }

    /// Indicates the number of repeats a pattern can match
    ///
    /// This is the greedy variant, matching as many as possible
    ///
    /// Uses u16 values to fit in a single word on 64-bit systems
    pub enum Range {
        /// Zero or more times. Same as `*` or `{0,}`
        FromZero,
        /// One or more times. Same as `+` or `{1,}`
        FromOne,
        /// <x> or more times (inclusive). Same as `{<x>,}`
        From(u16),
        /// Zero to <x> times (inclusive). Same as `{,<x>}`
        To(u16),
        /// Exactly <x> times. Same as `{<x>}`
        Times(u16),
        /// Between <x> and <y> times (inclusive). Same as `{<x>,<y>}`
        Range(u8, u8),
    }

    pub enum MatchStatus {
        NoMatch,
        Match,
        Consumed,
        RetroMatch(u16),
    }

    pub trait Matchable<Other=Self> {
        fn matches(&mut self, other: &Other) -> MatchStatus { NoMatch }
        #[inline] fn try_reset(&mut self) { }
    }

    pub trait Matcher<T : Matchable> {
        fn consume(&mut self, next: &T) -> MatchStatus { NoMatch }
        #[inline] fn is_enabled(&self) -> bool { true }
        fn enable(&mut self);
        fn disable(&mut self);
        #[inline] fn reset(&mut self) {
            self.enable();
        }
    }

    pub trait Matcherable<T : Matchable, This = Self> where Self : Matcher<T> {}
    impl<T : Matchable<T>, This : Matcher<T>> Matcherable<T, This> for This {}


    pub enum SimpleMatcher<'a, T : Matchable> {
        Unary(&'a mut Unary<T>),
        Binary(&'a mut Ordered<T, 2>),
        Ternary(&'a mut Ordered<T, 3>),
        Quartary(&'a mut Ordered<T, 4>),
        Optional(&'a mut SimpleMatcher<'a, T>),
        Repeated(&'a mut SimpleMatcher<'a, T>, Range),
        Lazy(&'a mut SimpleMatcher<'a, T>, LazyRange),
    }
    impl<'a, T : Matchable> Matcher<T> for SimpleMatcher<'a, T> {
        fn enable(&mut self) {
            match self {
                Self::Unary(matcher) => matcher.enable(),
                Self::Binary(matcher) => matcher.enable(),
                Self::Ternary(matcher) => matcher.enable(),
                Self::Quartary(matcher) => matcher.enable(),
                Self::Optional(matcher) => matcher.enable(),
                Self::Repeated(matcher, ..) => matcher.enable(),
                Self::Lazy(matcher, ..) => matcher.enable(),
            }
        }
        fn disable(&mut self) {
            match self {
                Self::Unary(matcher) => matcher.disable(),
                Self::Binary(matcher) => matcher.disable(),
                Self::Ternary(matcher) => matcher.disable(),
                Self::Quartary(matcher) => matcher.disable(),
                Self::Optional(matcher) => matcher.disable(),
                Self::Repeated(matcher, ..) => matcher.disable(),
                Self::Lazy(matcher, ..) => matcher.disable(),
            }
        }
        fn consume(&mut self, next: &T) -> MatchStatus {
            match self {
                Self::Unary(matcher) => matcher.consume(next),
                Self::Binary(matcher) => matcher.consume(next),
                Self::Ternary(matcher) => matcher.consume(next),
                Self::Quartary(matcher) => matcher.consume(next),
                Self::Optional(matcher) => matcher.consume(next),
                Self::Repeated(matcher, ..) => matcher.consume(next),
                Self::Lazy(matcher, ..) => matcher.consume(next),
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
        fn is_enabled(&self) -> bool {
            match self {
                Self::Unary(matcher) => matcher.is_enabled(),
                Self::Binary(matcher) => matcher.is_enabled(),
                Self::Ternary(matcher) => matcher.is_enabled(),
                Self::Quartary(matcher) => matcher.is_enabled(),
                Self::Optional(matcher) => matcher.is_enabled(),
                Self::Repeated(matcher, ..) => matcher.is_enabled(),
                Self::Lazy(matcher, ..) => matcher.is_enabled(),
            }
        }
    }
    impl<'a, T : Matchable> Matchable<T> for SimpleMatcher<'a, T> {
        fn matches(&mut self, other: &T) -> MatchStatus {
            self.consume(other)
        }
    }

    pub mod extensions {
        use std::marker::PhantomData;
        use super::*;

        pub enum BinaryCompound<'a, T : Matchable, MA : Matcherable<T>, MB : Matcherable<T>> {
            MatcherSetA(&'a mut MA),
            MatcherSetB(&'a mut MB),
            #[deprecated(note = "Do not use this; Only present for type enforcement")]
            _Phantom(PhantomData<T>)
        }
        impl<'a, T : Matchable, MA : Matcherable<T>, MB : Matcherable<T>> Matcher<T> for BinaryCompound<'a, T , MA, MB> {
            fn enable(&mut self) {
                match self {
                    Self::MatcherSetA(matcher) => matcher.enable(),
                    Self::MatcherSetB(matcher) => matcher.enable(),
                    _ => panic!()
                }
            }
            fn disable(&mut self) {
                match self {
                    Self::MatcherSetA(matcher) => matcher.disable(),
                    Self::MatcherSetB(matcher) => matcher.disable(),
                    _ => panic!()
                }
            }
            fn consume(&mut self, next: &T) -> MatchStatus {
                match self {
                    Self::MatcherSetA(matcher) => matcher.consume(next),
                    Self::MatcherSetB(matcher) => matcher.consume(next),
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
            fn is_enabled(&self) -> bool {
                match self {
                    Self::MatcherSetA(matcher) => matcher.is_enabled(),
                    Self::MatcherSetB(matcher) => matcher.is_enabled(),
                    _ => panic!()
                }
            }
        }
        impl<'a, T : Matchable, MA : Matcherable<T>, MB : Matcherable<T>> Matchable<T> for BinaryCompound<'a, T , MA, MB> {
            fn matches(&mut self, other: &T) -> MatchStatus {
                self.consume(other)
            }
        }

        pub enum TernaryCompound<'a, T : Matchable, MA : Matcherable<T>, MB : Matcherable<T>, MC : Matcherable<T>> {
            MatcherSetA(&'a mut MA),
            MatcherSetB(&'a mut MB),
            MatcherSetC(&'a mut MC),
            #[deprecated(note = "Do not use this; Only present for type enforcement")]
            _Phantom(PhantomData<T>)
        }
        impl<'a, T : Matchable, MA : Matcherable<T>, MB : Matcherable<T>, MC : Matcherable<T>> Matcher<T>
            for TernaryCompound<'a, T , MA, MB, MC> {
            fn enable(&mut self) {
                match self {
                    Self::MatcherSetA(matcher) => matcher.enable(),
                    Self::MatcherSetB(matcher) => matcher.enable(),
                    Self::MatcherSetC(matcher) => matcher.enable(),
                    _ => panic!()
                }
            }
            fn disable(&mut self) {
                match self {
                    Self::MatcherSetA(matcher) => matcher.disable(),
                    Self::MatcherSetB(matcher) => matcher.disable(),
                    Self::MatcherSetC(matcher) => matcher.disable(),
                    _ => panic!()
                }
            }
            fn consume(&mut self, next: &T) -> MatchStatus {
                match self {
                    Self::MatcherSetA(matcher) => matcher.consume(next),
                    Self::MatcherSetB(matcher) => matcher.consume(next),
                    Self::MatcherSetC(matcher) => matcher.consume(next),
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
            fn is_enabled(&self) -> bool {
                match self {
                    Self::MatcherSetA(matcher) => matcher.is_enabled(),
                    Self::MatcherSetB(matcher) => matcher.is_enabled(),
                    Self::MatcherSetC(matcher) => matcher.is_enabled(),
                    _ => panic!()
                }
            }
        }
        impl<'a, T : Matchable, MA : Matcherable<T>, MB : Matcherable<T>, MC : Matcherable<T>> Matchable<T>
            for TernaryCompound<'a, T , MA, MB, MC> {
            fn matches(&mut self, other: &T) -> MatchStatus {
                self.consume(other)
            }
        }

        pub type DynamicExtension<'a, T, M> = BinaryCompound<'a, T, M, &'a dyn Matcher<T>>;
        pub type DynamicDoubleExtension<'a, T, M> = TernaryCompound<'a, T, M, &'a dyn Matcher<T>, &'a dyn Matcher<T>>;
        pub type NOrderedExtension<'a, T, M, const LEN: usize> = BinaryCompound<'a, T, M, &'a mut Ordered<T, LEN>>;
        pub type NOrderedDoubleExtension<'a, T, M, const LEN_A: usize, const LEN_B: usize>
            = TernaryCompound<'a, T, M, &'a mut Ordered<T, LEN_A>, &'a mut Ordered<T, LEN_B>>;
    }

    pub struct Unary<T : Matchable> {
        enabled: bool,
        element: T,
    }
    impl<T : Matchable> Matcher<T> for Unary<T> {
        fn consume(&mut self, next: &T) -> MatchStatus {
            match self.element.matches(next) {
                status @ ( Match | Consumed) => status,
                MatchStatus::NoMatch => {
                    self.enabled = false;
                    NoMatch
                }
            }
        }
        #[inline] fn is_enabled(&self) -> bool { self.enabled }
        #[inline] fn enable(&mut self) { self.enabled = true }
        #[inline] fn disable(&mut self) { self.enabled = false }
        #[inline] fn reset(&mut self) {
            self.enabled = true;
            self.element.try_reset()
        }
    }
    impl<T: Matchable> Matchable<T> for Unary<T> {
        #[inline] fn matches(&mut self, next: &T) -> MatchStatus {
            self.element.matches(next)
        }
        #[inline]
        fn try_reset(&mut self) { self.reset() }
    }

    pub struct Ordered<T : Matchable, const LEN: usize> {
        cursor: usize,
        enabled: bool,
        elements: [T; LEN],
    }
    impl<T : Matchable, const LEN: usize> Matcher<T> for Ordered<T, LEN> {
        #[inline]
        fn consume(&mut self, next: &T) -> MatchStatus {
            if !self.enabled {
                return NoMatch
            }
            if LEN - 1 == self.cursor {
                self.elements[self.cursor].matches(next)
            } else {
                match self.elements[self.cursor].matches(next) {
                    Match => {
                        self.cursor += 1;
                        MatchStatus::Consumed
                    }
                    Consumed => {
                        MatchStatus::Consumed
                    }
                    RetroMatch(num) => {
                        RetroMatch(num)
                    }
                    NoMatch => {
                        self.enabled = false;
                        MatchStatus::NoMatch
                    }
                }
            }
        }
        #[inline] fn is_enabled(&self) -> bool { self.enabled }
        #[inline] fn enable(&mut self) { self.enabled = true }
        #[inline] fn disable(&mut self) { self.enabled = false }
        #[inline]
        fn reset(&mut self) {
            self.enabled = true;
            self.cursor = 0;
            for i in 0..LEN {
                self.elements[i].try_reset()
            }
        }
    }
    impl<T: Matchable, const LEN: usize> Matchable<T> for Ordered<T, LEN> {
        #[inline]
        fn matches(&mut self, next: &T) -> MatchStatus {
            let mut highest_match = MatchStatus::NoMatch;
            for i in 0..LEN {
                match self.elements[i].matches(&next) {
                    MatchStatus::Match => return MatchStatus::Match,
                    MatchStatus::Consumed => highest_match = MatchStatus::Consumed,
                    MatchStatus::NoMatch => ()
                }
            }
            highest_match
        }
        #[inline]
        fn try_reset(&mut self) {
            self.reset()
        }
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {

    }
}
