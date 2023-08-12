use std::array::IntoIter;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use crate::parse::expr::Expr;
use crate::matcher::MatcherType;
use crate::modifier::ModifierType;

pub(crate) struct RootCollection<'a> {
    roots: Vec<Root>,
    _phantom: PhantomData<&'a ()>
}
impl<'a> From<Vec<Root>> for RootCollection<'a> {
    fn from(roots: Vec<Root>) -> RootCollection<'a> {
        RootCollection {
            roots,
            _phantom: Default::default(),
        }
    }
}
impl<'a> IntoIterator for &'a RootCollection<'a> {
    type Item = RootCollectionIterResult<'a>;
    type IntoIter = RootCollectionIter<'a>;

    fn into_iter(self) -> RootCollectionIter<'a> {
        RootCollectionIter::<'a> {
            current_index: 0,
            current_sub_index: 0,
            roots: &self.roots,
            parent_root: None,
        }
    }
}
impl<'a> Debug for RootCollection<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for res in self.into_iter() {
            match res {
                RootCollectionIterResult::EndRoot => continue,
                RootCollectionIterResult::Src(src) => write!(f, "{}", src)?,
                RootCollectionIterResult::SubRoot(root) => write!(f, "{:?}", root)?,
                RootCollectionIterResult::Sustain(root) => write!(f, "{:?}", root)?,
            }
        }
        Ok(())
    }
}

pub(crate) enum RootCollectionIterResult<'a> {
    EndRoot,
    Src(&'a String),
    SubRoot(&'a Root),
    Sustain(&'a Root),
}

pub(crate) struct RootCollectionIter<'a> {
    current_index: usize,
    current_sub_index: usize,
    parent_root: Option<&'a Root>,
    roots: &'a Vec<Root>,
}
impl<'a> Iterator for RootCollectionIter<'a> {
    type Item = RootCollectionIterResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_sub_index += 1;
        match self.parent_root?.branches.get(self.current_sub_index.clone()) {
            None => match &self.parent_root?.parent {
                Some(i) => {
                    self.parent_root = Some(&self.roots[i.clone()]);
                    Some(RootCollectionIterResult::EndRoot)
                },
                None => None,
            },
            Some(branch) => {
                match branch {
                    BranchValue::Root(i) => {
                        Some(RootCollectionIterResult::Sustain(&self.roots[i.clone()]))
                    }
                    BranchValue::Source(src) => Some(RootCollectionIterResult::Src(src))
                }
            }
        }
    }
}

#[derive(Debug)]
pub(crate) enum RootType {
    Modifier(ModifierType),
    Matcher(MatcherType),
    Root
}

#[derive(Debug)]
pub(crate) enum BranchValue {
    Root(usize),
    Source(String),
}

#[derive(Debug)]
pub(crate) struct Root {
    pub(crate) branches: Vec<BranchValue>,
    pub(crate) root_type: RootType,
    pub(crate) args: Vec<Expr>,
    pub(crate) parent: Option<usize>,
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
        }
    }
}