use crate::code_analysis::{
    syntax::Token, 
    text::SourceText
};

pub struct Lexer {
    source: SourceText
}

impl Lexer {
    pub fn from(source: SourceText) -> Lexer {
        Lexer {
            source
        }
    }
    pub fn lex(&mut self) -> Token {
        unimplemented!()
    }
}