#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, salsa::Update)]
#[repr(u16)]
pub enum SyntaxKind {
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
    NL,
    /// `;`
    Semi,
    /// `,`
    Comma,
    /// `:`
    Colon,
    /// `.`
    Dot,
    /// `..`
    Dot2,
    /// `...`
    Dot3,
    /// `..=`
    Dot2Eq,

    /// `&`
    Amp,
    /// `|`
    Pipe,
    /// `=`
    Eq,
    /// `!`
    Excl,
    /// `!=`
    Ne,
    /// `>`
    Gt,
    /// `>=`
    Ge,
    /// `<`
    Lt,
    /// `<=`
    Le,
    /// `-`
    Minus,
    /// `+`
    Plus,
    /// `/`
    Slash,
    /// `*`
    Star,
    /// `%`
    Percent,
    /// `~`
    Tilde,
    /// `<<`
    Shl,
    /// `>>`
    Shr,
    /// `<=>`
    Spaceship,

    /// `:=`
    Assign,
    /// `&=`
    AmpEq,
    /// `|=`
    PipeEq,
    /// `-=`
    MinusEq,
    /// `+=`
    PlusEq,
    /// `/=`
    SlashEq,
    /// `*=`
    StarEq,
    /// `%=`
    PercentEq,
    /// `~=`
    TildeEq,
    /// `<<=`
    ShlEq,
    /// `>>=`
    ShrEq,

    /// `++`
    Inc,
    /// `--`
    Dec,

    /// `->`
    Arrow,
    /// `?`
    Quest,
    /// `::`
    Colon2,

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

    /// `as`
    AsKw,
    /// `true`
    TrueKw,
    /// `false`
    FalseKw,
    /// `enum`
    EnumKw,
    /// `class`
    ClassKw,
    /// `struct`
    StructKw,
    /// `fn`
    FnKw,
    /// `egg`
    EggKw,
    /// `pub`
    PubKw,
    /// `mut`
    MutKw,
    /// `throw`
    ThrowKw,
    /// `catch`
    CatchKw,
    /// `with`
    WithKw,
    /// `const`
    ConstKw,
    /// `use`
    UseKw,
    /// `type`
    TypeKw,
    /// `return`
    ReturnKw,
    /// `extend`
    ExtendKw,
    /// `trait`
    TraitKw,

    /// An identifier or soft keyword e.g. `foo` or `class`.
    Ident,
    /// An escaped identifier, e.g. `` `ident` ``
    RawIdent,

    /// An integer literal constant, e.g. `123` or `0xFF`.
    IntLit,
    /// A floating-point literal constant, e.g. `1.0` or `1e9`.
    FloatLit,

    /// Unknown token, not expected by the lexer, e.g. "№".
    Unknown,

    /// End of input.
    EOF,

    // Nodes
    ClassDecl,
    StructDecl,
    EnumDecl,
    FnDecl,
    ParenExpr,
    LitConstExpr,
    ExprStmt,
    File,
}
