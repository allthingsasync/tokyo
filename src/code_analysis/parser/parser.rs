use crate::code_analysis::syntax::NT;

use super::LexBuffer;

pub struct Parser {
    tokens: LexBuffer
}

impl Parser {
    pub fn new(source: String) -> Parser {
        unimplemented!()
    }

    pub fn parse_text(text: String) -> NT {
        let mut p = Parser::new(text);
        p.parse()
    }

    pub fn parse(&mut self) -> NT {
        unimplemented!()
    }
}