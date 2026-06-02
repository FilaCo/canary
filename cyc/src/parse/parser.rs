use crate::{
    db::CanaryDb,
    ir::{
        source::SourceFile,
        syntax::{CanaryFile, Stmt},
    },
    parse::{Lexer, Token, TokenType},
};

pub struct Parser<'db> {
    db: &'db dyn CanaryDb,
    lexer: Lexer<'db>,
    file: SourceFile,
}

impl<'db> Parser<'db> {
    pub fn new(db: &'db dyn CanaryDb, file: SourceFile) -> Self {
        todo!()
    }

    /// ```ebnf
    /// canary_file = { NL } { stmt semi } EOF .
    /// ```
    pub fn canary_file(&mut self) -> CanaryFile<'db> {
        while self.eat(TokenType::NL) {}

        let mut stmts = Vec::new();
        while !self.at(TokenType::EOF) {
            let stmt = self.stmt();
            stmts.push(stmt);
            self.semi();
        }

        self.expect(TokenType::EOF);

        CanaryFile::new(self.db, stmts)
    }

    fn stmt(&mut self) -> Stmt<'db> {
        todo!()
    }

    /// ```ebnf
    /// semi = ( ";" | NL ) { NL } .
    /// ```
    fn semi(&mut self) -> Option<Token<'db>> {
        if !self.at_any(&[TokenType::Semi, TokenType::NL]) {
            return None;
        }

        let tok = self.bump();
        while self.eat(TokenType::NL) {}

        Some(tok)
    }

    fn expect(&mut self, expected: TokenType) -> Option<Token<'db>> {
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
        expected.contains(&TokenType::from(self.first().kind))
    }

    fn at(&self, expected: TokenType) -> bool {
        TokenType::from(self.first().kind) == expected
    }

    fn first(&self) -> &Token<'db> {
        self.lexer.first()
    }

    fn bump(&mut self) -> Token<'db> {
        self.lexer.bump()
    }
}
