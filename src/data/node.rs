// Original file: "Node.hs"
// File auto-generated using Corollary.

use std::fmt;
use std::rc::Rc;

use data::name::Name;
use data::position::{Pos, Position, PosLength};

// a class for convenient access to the attributes of an attributed object
pub trait CNode {
    fn node_info(&self) -> &NodeInfo;
    fn into_node_info(self) -> NodeInfo;
}

impl CNode for NodeInfo {
    fn node_info(&self) -> &NodeInfo {
        self
    }
    fn into_node_info(self) -> NodeInfo {
        self
    }
}

impl<T: CNode + Clone> CNode for Rc<T> {
    fn node_info(&self) -> &NodeInfo {
        (**self).node_info()
    }
    fn into_node_info(self) -> NodeInfo {
        (*self).node_info().clone()
    }
}

impl<T: CNode> Pos for T {
    fn pos(&self) -> Rc<Position> {
        NodeInfo::pos(self.node_info())
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum NodeInfo {
    OnlyPos(Rc<Position>, Rc<Position>, usize),
    NodeInfo(Rc<Position>, Rc<Position>, usize, Name),
}
pub use self::NodeInfo::*;

// TODO This should be replaced with a better impl
impl fmt::Debug for NodeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "..")
    }
}

impl NodeInfo {
    pub fn internal() -> NodeInfo {
        Self::undef()
    }

    pub fn undef() -> NodeInfo {
        let undef = Rc::new(Position::none());
        OnlyPos(undef.clone(), undef, 0)
    }

    pub fn new(pos1: Rc<Position>, pos2: Rc<Position>, len: usize, name: Name) -> NodeInfo {
        NodeInfo(pos1, pos2, len, name)
    }

    pub fn with_only_pos(pos: Rc<Position>) -> NodeInfo {
        OnlyPos(pos, Rc::new(Position::none()), 0)
    }

    pub fn with_pos_len(a: Rc<Position>, b: Rc<Position>, len: usize) -> NodeInfo {
        OnlyPos(a, b, len)
    }

    pub fn with_pos_name(pos: Rc<Position>, name: Name) -> NodeInfo {
        NodeInfo(pos, Rc::new(Position::none()), 0, name)
    }

    pub fn len(&self) -> Option<usize> {
        match *self {
            NodeInfo(ref first_pos, ref last_pos, last_len, _) |
            OnlyPos(ref first_pos, ref last_pos, last_len) => if last_len == 0 {
                None
            } else {
                Some(last_pos.offset().unwrap() + last_len - first_pos.offset().unwrap())
            }
        }
    }

    pub fn get_last_token_pos(&self) -> PosLength {
        match *self {
            NodeInfo(_, ref last_pos, last_len, _) |
            OnlyPos(_, ref last_pos, last_len) => (last_pos.clone(), last_len),
        }
    }

    pub fn name(&self) -> Option<Name> {
        match *self {
            OnlyPos(..) => None,
            NodeInfo(_, _, _, name) => Some(name),
        }
    }

    // NOTE: this is not an impl of Pos because that impl is automatic
    // for all CNodes and falls back to this inherent method!

    fn pos(&self) -> Rc<Position> {
        match *self {
            NodeInfo(ref pos, ..) | OnlyPos(ref pos, ..) => pos.clone(),
        }
    }
}
