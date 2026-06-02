// #[derive(Clone, Copy, Debug)]
// enum TokenKind<'a> {
//     Minus,
//     Plus,
//     Slash,
//     Star,
//     LParen,
//     RParen,
//     Int { value: &'a str },
//     Dummy,
//     Eof,
// }

// use TokenKind::*;

// #[derive(Clone, Copy, Debug)]
// struct Token<'a> {
//     kind: TokenKind<'a>,
//     span: Span,
// }

// impl<'a> Token<'a> {
//     fn new(kind: TokenKind<'a>, span: Span) -> Self {
//         Self { kind, span }
//     }

//     fn dummy() -> Self {
//         Self::new(Dummy, Span::dummy())
//     }

//     fn glue(&self, _: &Self) -> Option<Self> {
//         None
//     }
// }

// #[derive(Debug)]
// struct Lexer<'a> {
//     input: Chars<'a>,
//     pos: usize,
//     token: Token<'a>,
// }

// impl<'a> Lexer<'a> {
//     pub fn new(input: &'a str) -> Self {
//         let mut lexer = Self {
//             input: input.chars(),
//             pos: 0,
//             token: Token::dummy(),
//         };
//         let _ = lexer.bump();
//         lexer
//     }

//     pub fn first(&self) -> Token<'a> {
//         self.token
//     }

//     pub fn bump(&mut self) -> Token<'a> {
//         let next_tok = loop {
//             let (next_tok, is_next_tok_preceded_by_ws) = self.next_token_from_input();

//             if is_next_tok_preceded_by_ws {
//                 break next_tok;
//             } else if let Some(glued) = self.token.glue(&next_tok) {
//                 self.token = glued;
//             } else {
//                 break next_tok;
//             }
//         };
//         std::mem::replace(&mut self.token, next_tok)
//     }

//     fn next_token_from_input(&mut self) -> (Token<'a>, bool) {
//         let mut preceeded_by_ws = false;
//         loop {
//             let str_before = self.input.as_str();
//             let start = self.pos;
//             let Some(first_char) = self.bump_char() else {
//                 return (Token::new(Eof, self.mk_sp(start)), preceeded_by_ws);
//             };

//             let kind = match first_char {
//                 c if is_whitespace(c) => {
//                     self.ws();
//                     preceeded_by_ws = true;
//                     continue;
//                 }
//                 '0'..='9' => self.int(str_before, start),
//                 '-' => Minus,
//                 '+' => Plus,
//                 '/' => Slash,
//                 '*' => Star,
//                 '(' => LParen,
//                 ')' => RParen,
//                 _ => {
//                     preceeded_by_ws = true;
//                     continue;
//                 }
//             };

//             let span = self.mk_sp(start);
//             return (Token::new(kind, span), preceeded_by_ws);
//         }
//     }

//     fn ws(&mut self) {
//         self.eat_char_while(is_whitespace)
//     }

//     fn int(&mut self, str_before: &'a str, start: usize) -> TokenKind<'a> {
//         self.eat_dec_digits();
//         let end = self.pos - start;
//         Int {
//             value: &str_before[..end],
//         }
//     }

//     fn eat_dec_digits(&mut self) {
//         loop {
//             match self.first_char() {
//                 '0'..='9' | '_' => {
//                     self.bump_char();
//                 }
//                 _ => break,
//             }
//         }
//     }

//     fn mk_sp(&self, start: usize) -> Span {
//         Span::new(start, self.pos)
//     }

//     fn eat_char_while(&mut self, predicate: impl Fn(char) -> bool) {
//         while !self.is_at_eof() && predicate(self.first_char()) {
//             self.bump_char();
//         }
//     }

//     fn bump_char(&mut self) -> Option<char> {
//         let c = self.input.next();
//         self.pos += c.map(|c| c.len_utf8()).unwrap_or(0);
//         c
//     }

//     fn first_char(&self) -> char {
//         self.input.clone().next().unwrap_or(EOF_CHAR)
//     }

//     fn is_at_eof(&self) -> bool {
//         self.input.as_str().is_empty()
//     }
// }

// const EOF_CHAR: char = '\0';

// fn is_whitespace(c: char) -> bool {
//     matches!(
//         c,
//         | '\u{0020}' // space
//         | '\u{0009}' // tab
//         | '\u{000C}' // form feed
//     )
// }

// impl<'a> Iterator for Lexer<'a> {
//     type Item = Token<'a>;

//     fn next(&mut self) -> Option<Self::Item> {
//         let t = self.bump();
//         match t.kind {
//             Eof => None,
//             _ => Some(t),
//         }
//     }
// }
