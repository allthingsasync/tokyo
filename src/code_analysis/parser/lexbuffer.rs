use crate::code_analysis::{
    syntax::Token, 
    text::SourceText
};

use super::Lexer;

pub struct LexBuffer {
    tokens: Vec<Token>,
    idx: usize,
    length: usize,
}

impl LexBuffer {
    pub fn for_lexer(lexer: Lexer) -> LexBuffer {
        unimplemented!()
    }

    pub fn for_source(source: SourceText) -> LexBuffer {
        unimplemented!()
    }
}