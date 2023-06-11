use std::marker::PhantomData;
use crate::matcher::matcher;
use crate::matcher::matcher::Matcher;
use crate::util::MatchStatus::*;

trait Executor<R, K> {
    fn execute(&mut self, next: &K) -> R;
}

struct RegisteredPatternExecutor<T : Matcher<K>, K, R, const PATTERNS : usize> {
    registered_patterns : [T; PATTERNS],
    _phantom : PhantomData<K>,
    _phantom2 : PhantomData<R>,
}

impl<T : Matcher<K>, K, R, const PATTERNS : usize> Executor<R, K> for RegisteredPatternExecutor<T, K, R, PATTERNS> {
    fn execute(&mut self, next: &K) -> R {
        for i in 0..PATTERNS {
            let pattern = &mut self.registered_patterns[i];
            match pattern.matches(next) {
                Match => todo!(),
                GreedyMatch => todo!(),
                LazyMatch => todo!(),
                Consumed => todo!(),
                NoMatch => todo!(),
            }
        }

        todo!()
    }
}