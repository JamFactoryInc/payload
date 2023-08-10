use std::cell::RefCell;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::{mem, ptr};
use std::ops::{ControlFlow, Deref, FromResidual, Try};
use crate::expr::{Expr, ExprParser};
use crate::matcher::MatcherType;
use crate::modifier::ModifierType;
use crate::parse::ParseResult::*;
use crate::root::{BranchValue, Root, RootType};






