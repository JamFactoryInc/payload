use crate::expr::Expr;
use crate::matcher::MatcherType;
use crate::modifier::ModifierType;

pub(crate) enum RootType {
    Modifier(ModifierType),
    Matcher(MatcherType),
    Root
}

pub(crate) enum BranchValue {
    Root(usize),
    Source(String),
}

pub(crate) struct Root {
    pub(crate) branches: Vec<BranchValue>,
    pub(crate) root_type: RootType,
    pub(crate) args: Vec<Expr>,
    pub(crate) parent: Option<usize>,
    pub(crate) index: usize,
}
impl Root {
    pub(crate) fn add_arg(&mut self, arg: Expr) {
        self.args.push(arg)
    }
}
impl Default for Root {
    fn default() -> Self {
        Root {
            branches: Vec::new(),
            root_type: RootType::Root,
            args: Vec::new(),
            parent: None,
            index: 0,
        }
    }
}