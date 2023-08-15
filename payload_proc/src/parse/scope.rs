use paste::paste;

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
        use transitions
            if (b'.')
                move from Default to LeadingDot
                then return Continue.
            if (b'.')
                move from Sep to LeadingDot
                then return Continue.
        for states
            Default,
            Sep,
            LeadingDot,
            StepDown,
            StepUp,
            PathName(String),
            Wildcard,
            DoubleWildcard
        with elements
            A
}
struct Scope {

}