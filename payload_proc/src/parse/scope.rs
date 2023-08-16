
// enum ScopeElement {
//     //  ./
//     // access a path one lower than the current one
//     StepDown,
//     //  ../
//     // access the path one higher than the current one
//     StepUp,
//     //  name
//     // match the exact path given
//     PathName(String),
//     //  /*
//     // match any single path
//     Wildcard,
//     //  /**
//     // match any series of paths
//     DoubleWildcard,
//
// }

enum ScopeParseState {
    LeadingPeriod,
    AwaitingPathBody,

}

// struct ScopeParser {
//     state: ScopeParseState
// }

stateful_parser!{
    for struct Scope
        states
            Default,
            Sep,
            LeadingDot,
            DoubleDot,
            StepDown,
            StepUp,
            PathName(String),
            Wildcard,
            DoubleWildcard
        transitions
            [Default | Sep] match b'.' => [LeadingDot] -> Continue;
            [Default] match b'/' => [Sep] -> Continue;
            [Default] match b'*' => [Wildcard] -> Continue;
            [Wildcard] match b'*' => [DoubleWildcard] -> Continue;
            [LeadingDot] match b'.' => [DoubleDot] -> Continue;
            [LeadingDot] match b'/' => [StepDown] -> Continue;
            [DoubleDot] match b'/' => [StepDown] -> Continue;
            [DoubleDot] match b'.' => "Illegal extra '.'";
            [Sep] match b'/' => "";
        elements
            Wildcard,
            DoubleWildcard,
            StepUp,
            StepDown


}
struct Scope {

}