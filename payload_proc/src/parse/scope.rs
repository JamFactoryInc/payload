
enum ScopeElement {
    //  ./
    // access a path one lower than the current one
    StepDown,
    //  ../
    // access the path one higher than the current one
    StepUp,
    //  name
    // match the exact path given
    PathName(String),
    //  /*
    // match any single path
    Wildcard,
    //  /**
    // match any series of paths
    DoubleWildcard,

}

enum ScopeParseState {
    LeadingPeriod,
    AwaitingPathBody,

}

struct ScopeParser {
    state: ScopeParseState
}

struct ScopeExpression(Vec<ScopeElement>);