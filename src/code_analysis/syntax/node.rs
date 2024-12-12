use super::{SyntaxKind, NT};

pub struct Node {
    kind: SyntaxKind,
    length: u32,
    children: Vec<NT>
}