pub mod unary_pattern;

use crate::payload_engine::lexer::token::Token;

pub trait Pattern {
    fn is_enabled(&self) -> bool;
    /// Takes in the current and next token and returns a MatchStatus
    ///
    /// If the MatchStatus is Nil or RetroComplete, the pattern should automatically be disabled
    ///
    /// The pattern might be disabled if Complete is returned and the pattern cannot continue
    ///
    /// This is what each match status means here:
    ///
    /// Nil: The pattern is no longer applicable and will be disabled
    /// until the parser re-enables it after finding a match & resetting
    ///
    /// Consumed: The pattern is continuing to match, but has not finished
    ///
    /// Complete: The most recent 'current' token completed the pattern. yay!
    /// This indicates to the parser that there are outstanding complete patterns
    ///
    /// RetroComplete: The current pattern has passed a potential match but continued to match.
    /// This match status indicates the just-consumed 'current' token broke the pattern, and the
    /// parser should step back to the last successful match. This may or may not be this pattern,
    /// as it rolls back to whichever pattern has the longest match
    ///
    fn consume(&mut self, current : Token, next : Token) -> MatchStatus;
    /// Resets all states to the original values. Effectively a new instance
    fn reset(&mut self);
}

pub enum MatchStatus {
    Nil,
    Consumed,
    Complete,
    RetroComplete,
}