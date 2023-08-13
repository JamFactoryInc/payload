use std::array::IntoIter;
use std::fmt::{Debug, Formatter, write};
use std::marker::PhantomData;
use crate::parse::expr::Expr;
use crate::matcher::MatcherType;
use crate::modifier::ModifierType;
use crate::root::RootType::{Matcher, Modifier};

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
            current_root_depth: 0,
            current_sub_index: 0,
            roots: &self.roots,
            parent_root: self.roots.first(),
        }
    }
}
impl<'a> Debug for RootCollection<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for res in self.into_iter() {
            match res {
                RootCollectionIterResult::EndRoot => continue,
                RootCollectionIterResult::Src {
                    source: src,
                    depth
                } => writeln!(f, "{:ident$}{}", "", src, ident=depth * 4)?,
                RootCollectionIterResult::SubRoot {
                    found_root: root,
                    depth
                } => writeln!(f, "{:ident$}{:?}", "", root, ident=depth * 4)?,
                RootCollectionIterResult::Sustain {
                    found_root: root,
                    depth
                } => writeln!(f, "{:ident$}{:?}", "", root, ident=depth * 4)?,
            }
        }
        Ok(())
    }
}

pub(crate) enum RootCollectionIterResult<'a> {
    EndRoot,
    Src { source: &'a String, depth: usize },
    SubRoot { found_root: &'a Root, depth: usize },
    Sustain { found_root: &'a Root, depth: usize },
}

#[derive(Debug)]
pub(crate) struct RootCollectionIter<'a> {
    current_root_depth: usize,
    current_sub_index: usize,
    parent_root: Option<&'a Root>,
    roots: &'a Vec<Root>,
}
impl<'a> Iterator for RootCollectionIter<'a> {
    type Item = RootCollectionIterResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_branch_within_parent = self.parent_root?.branches.get(self.current_sub_index.clone());
        let result = match current_branch_within_parent {
            // we've exhausted all branches within the current parent
            None => match &self.parent_root?.parent_info {
                // if the current parent has no parent, exit
                None => None,
                // step up to the current parent's parent
                Some(ParentInfo {
                    index_of_parent,
                    index_within_parent
                }) => {
                    self.parent_root = Some(&self.roots[index_of_parent.clone()]);
                    self.current_root_depth -= 1;
                    self.current_sub_index = index_within_parent.clone();
                    Some(RootCollectionIterResult::EndRoot)
                },
            },
            Some(branch) => match branch {
                BranchVariant::Source(src) => Some(RootCollectionIterResult::Src {
                    source: src,
                    depth: self.current_root_depth.clone()
                }),
                BranchVariant::Root(i) => {
                    let found_root = &self.roots[i.clone()];
                    if found_root.branches.is_empty() {
                        Some(RootCollectionIterResult::Sustain {
                            found_root: found_root,
                            depth: self.current_root_depth.clone()
                        })
                    } else {
                        self.parent_root = Some(found_root);
                        let new_parent_first_child_root = self.parent_root.unwrap().branches.first();
                        self.current_root_depth += 1;
                        match new_parent_first_child_root {
                            Some(BranchVariant::Root(i)) =>
                                Some(RootCollectionIterResult::SubRoot {
                                    found_root: &self.roots[i.clone()],
                                    depth: self.current_root_depth.clone()
                                }),
                            Some(BranchVariant::Source(src)) =>
                                Some(RootCollectionIterResult::Src {
                                    source: src,
                                    depth: self.current_root_depth.clone()
                                }),
                            _ => None
                        }
                    }
                }
            }
        };
        self.current_sub_index += 1;
        result
    }
}

#[derive(Clone)]
pub(crate) enum RootType {
    Modifier(ModifierType),
    Matcher(MatcherType),
    Root
}
impl RootType {
    pub fn modifier() -> RootType {
        Modifier(ModifierType::Nil)
    }

    pub fn matcher() -> RootType {
        Matcher(MatcherType::Nil)
    }
}
impl Debug for RootType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Modifier(modifier) => write!(f, "{:?}", modifier)?,
            Matcher(matcher) => write!(f, "{:?}", matcher)?,
            RootType::Root => write!(f, "<root>")?,
        }
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) enum BranchVariant {
    Root(usize),
    Source(String),
}

#[derive(Debug)]
pub(crate) struct ParentInfo {
    pub(crate) index_of_parent: usize,
    pub(crate) index_within_parent: usize,
}

pub(crate) struct Root {
    pub(crate) branches: Vec<BranchVariant>,
    pub(crate) root_type: RootType,
    pub(crate) args: Vec<Expr>,
    pub(crate) parent_info: Option<ParentInfo>,
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
            parent_info: None,
        }
    }
}
impl Debug for Root {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | {:?}",
            match self.parent_info {
                Some(ParentInfo{
                    index_of_parent: _,
                    index_within_parent
                }) => index_within_parent.to_string(),
                None => "".to_string()
            },
            self.root_type
        )?;

        if !self.args.is_empty() {
            write!(f, "({:?})", self.args)?
        }

        if self.branches.is_empty() {
            Ok(())
        } else {
            writeln!(f, " {{")?;
            for branch in &self.branches {
                match branch {
                    BranchVariant::Root(i) => writeln!(f, "    {i}"),
                    BranchVariant::Source(src) => writeln!(f, "    {src:?}"),
                }?
            }
            writeln!(f, "}}")
        }
    }
}