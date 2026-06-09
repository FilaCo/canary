use cyc_ir::{
    source::BytePos,
    syntax::{CanaryFile, Expr, Stmt, StmtKind, Token},
};

use crate::parse::{Lexer, ParseSession};

#[derive(Debug)]
pub struct Parser<'ci> {
    psess: ParseSession<'ci>,
    lexer: Lexer<'ci>,
    token: Token,
}

impl<'ci> Parser<'ci> {
    pub fn new(src: &'ci str, start_pos: BytePos, psess: ParseSession<'ci>) -> Self {
        let mut lexer = Lexer::new(src, start_pos, psess);
        let token = lexer.bump();

        Self {
            psess,
            lexer,
            token,
        }
    }

    /// ```ebnf
    /// file = { NL } { stmt semi } EOF .
    /// ```
    pub fn parse_file(&mut self) -> CanaryFile {
        // while self.eat(TokenType::NL) {}

        // let mut stmts = Vec::new();
        // while !self.at(TokenType::EOF) {
        //     let stmt = self.stmt();
        //     stmts.push(stmt);
        //     self.semi();
        // }

        // self.expect(TokenType::EOF);

        todo!()
    }

    /// ```ebnf
    /// stmt = expr .
    /// ```
    fn parse_stmt(&mut self) -> Stmt {
        let expr = self.parse_expr(0);
        let span = expr.span;
        Stmt {
            kind: StmtKind::Expr(expr),
            span,
        }
    }

    fn parse_expr(&mut self, min_bp: u8) -> Expr {
        todo!()
    }

    /// ```ebnf
    /// semi = ( ";" | NL ) { NL } .
    /// ```
    fn parse_semi(&mut self) {
        todo!()
    }

    // fn expect(&mut self, expected: TokenType) -> Option<CanaryToken> {
    //     if self.at(expected) {
    //         return Some(self.bump());
    //     }

    //     None
    // }

    // fn eat(&mut self, expected: TokenType) -> bool {
    //     if self.at(expected) {
    //         self.bump();
    //         return true;
    //     }

    //     false
    // }

    // fn at_any(&self, expected: &[TokenType]) -> bool {
    //     expected.iter().any(|tt| *tt == self.first().kind)
    // }

    // fn at(&self, expected: TokenType) -> bool {
    //     self.first().kind == expected
    // }

    fn first(&self) -> &Token {
        &self.token
    }

    fn bump(&mut self) -> Token {
        std::mem::replace(&mut self.token, self.lexer.bump())
    }
}
