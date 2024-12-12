use super::{Node, Token};

pub enum NT {
    N(Node),
    T(Token)
}