use crate::{
    ir::{
        source::SourceFile,
        syntax::{CanaryFile, CanaryToken, Stmt},
    },
    parse::{Lexer, TokenType},
};

pub struct Parser<'src> {
    lexer: Lexer<'src>,
    file: SourceFile,
    token: CanaryToken,
}

impl<'db> Parser<'db> {
    pub fn new(file: SourceFile) -> Self {
        todo!()
    }

    /// ```ebnf
    /// canary_file = { NL } { stmt semi } EOF .
    /// ```
    pub fn canary_file(&mut self) -> CanaryFile {
        while self.eat(TokenType::NL) {}

        let mut stmts = Vec::new();
        while !self.at(TokenType::EOF) {
            let stmt = self.stmt();
            stmts.push(stmt);
            self.semi();
        }

        self.expect(TokenType::EOF);

        todo!()
    }

    fn stmt(&mut self) -> Stmt {
        todo!()
    }

    /// ```ebnf
    /// semi = ( ";" | NL ) { NL } .
    /// ```
    fn semi(&mut self) -> Option<CanaryToken> {
        if !self.at_any(&[TokenType::Semi, TokenType::NL]) {
            return None;
        }

        let tok = self.bump();
        while self.eat(TokenType::NL) {}

        Some(tok)
    }

    fn expect(&mut self, expected: TokenType) -> Option<CanaryToken> {
        if self.at(expected) {
            return Some(self.bump());
        }

        None
    }

    fn eat(&mut self, expected: TokenType) -> bool {
        if self.at(expected) {
            self.bump();
            return true;
        }

        false
    }

    fn at_any(&self, expected: &[TokenType]) -> bool {
        expected.iter().any(|tt| *tt == self.first().kind)
    }

    fn at(&self, expected: TokenType) -> bool {
        self.first().kind == expected
    }

    fn first(&self) -> &CanaryToken {
        &self.token
    }

    fn bump(&mut self) -> CanaryToken {
        std::mem::replace(&mut self.token, self.lexer.bump())
    }
}
