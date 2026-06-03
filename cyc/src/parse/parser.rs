use crate::ir::{
    source,
    syntax::{self, Expr, Stmt, Token},
};

pub struct Parser {
    // lexer: Lexer<'psess>,
    token: Token,
}

impl Parser {
    pub fn new(file: source::File) -> Self {
        todo!()
    }

    /// ```ebnf
    /// file = { NL } { stmt semi } EOF .
    /// ```
    pub fn parse_file(&mut self) -> syntax::File {
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
        todo!()
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
        todo!()
        // std::mem::replace(&mut self.token, self.lexer.bump())
    }
}
