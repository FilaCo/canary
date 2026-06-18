#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, salsa::Update)]
#[repr(u16)]
pub enum SyntaxKind {
    // Tokens
    /// A block comment, e.g. `/* block comment */`.
    ///
    /// Block comments can be recursive, so a sequence like `/* /* */`
    /// will not be considered terminated and will result in a parsing error.
    ///
    /// `"/*" { BlockComment | /* an arbitrary Unicode code point */ } "*/" .`
    BlockComment = 0,
    /// A line comment, e.g. `// comment`.
    ///
    /// `"//" { /* an arbitrary Unicode code point except LF and CR */ } .`
    LineComment,
    /// `/* one of the following Unicode code points: SPACE U+0020, TAB U+0009,
    /// Form Feed U+000C */ .`
    Whitespace,

    /// `LF | ( CR [ LF ] ) .`
    Newline,
    /// `;`.
    Semi,
    /// `,`.
    Comma,
    /// `:`.
    Colon,
    /// `.`.
    Dot,

    /// `=`
    Eq,
    /// `!`
    Excl,
    /// `>`
    Gt,
    /// `<`
    Lt,
    /// `-`
    Minus,
    /// `+`
    Plus,
    /// `/`
    Slash,
    /// `*`
    Star,
    /// `~`
    Tilde,

    /// `?`
    Quest,

    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `[`
    LBracket,
    /// `]`
    RBracket,
    /// `(`
    LParen,
    /// `)`
    RParen,

    /// An identifier or keyword e.g. `foo` or `class`.
    Ident,

    /// A literal constant value, e.g. `123` or `"hello"`.
    Literal,

    /// Unknown token, not expected by the lexer, e.g. "№".
    Unknown,

    /// End of input.
    Eof,
}
