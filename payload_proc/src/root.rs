use crate::expr::Expr;
use crate::matcher::MatcherType;
use crate::modifier::ModifierType;

pub(crate) enum RootType {
    Modifier(ModifierType),
    Matcher(MatcherType),
    Root
}

pub(crate) struct Root {
    pub(crate) branches: Vec<Root>,
    pub(crate) root_type: RootType,
    pub(crate) args: Vec<Expr>,
    pub(crate) modifiers: Vec<ModifierType>
}
impl Root {
    pub(crate) fn add(&mut self, root: Root) {
        self.branches.push(root)
    }

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
            modifiers: Vec::new(),
        }
    }
}