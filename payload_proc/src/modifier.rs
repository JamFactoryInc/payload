
pub(crate) enum ModifierType {
    Link{ other_scope: String },
    Type{ type_name: String },
    Bind{ field_name: String },
    Map{ mapping_logic: String},
    PreProcess{ src: String },
    PostProcess{ src: String },
    Naive,
    Symbol{ path: String, type_id: String },
    Suggest,
    Error{ message: String },
    Ignore,
    Exact,
    NotExact,
}