use super::SyntaxKind;

pub struct Token {
    kind: SyntaxKind,
    text: String,
}