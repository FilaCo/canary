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
    /// `==`
    Eq2,
    /// `===`
    Eq3,
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
    /// `<=>`
    Spaceship,

    /// `:=`
    Bind,
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
    /// `=>`
    FatArrow,
    /// `<:`
    Subtype,
    /// `_`
    Wildcard,

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
    /// `match`
    MatchKw,
    /// `if`
    IfKw,
    /// `else`
    ElseKw,
    /// `elif`
    ElifKw,
    /// `loop`
    LoopKw,
    /// `for`
    ForKw,
    /// `in`
    InKw,
    /// `while`
    WhileKw,
    /// `throws`
    ThrowsKw,
    /// `abstract`
    AbstractKw,
    /// `static`
    StaticKw,
    /// `open`
    OpenKw,
    /// `override`
    OverrideKw,
    /// `init`
    InitKw,
    /// `where`
    WhereKw,
    /// `super`
    SuperKw,
    /// `break`
    BreakKw,
    /// `continue`
    ContinueKw,

    /// An identifier or soft keyword e.g. `foo` or `class`.
    Ident,
    /// An escaped identifier, e.g. `` `ident` ``
    RawIdent,

    /// An integer literal constant, e.g. `123` or `0xFF`.
    IntLit,
    /// A floating-point literal constant, e.g. `1.0` or `1e9`.
    FloatLit,
    /// A string literal constant, e.g. `"foo"`.
    StringLit,

    /// Unknown token, not expected by the lexer, e.g. "№".
    Unknown,

    /// End of input.
    EOF,

    // Nodes
    Error,

    File,

    EggDecl,
    ClassDecl,
    TraitDecl,
    EnumDecl,
    FnDecl,
    InitDecl,
    BindDecl,
    TypeDecl,
    ExtendDecl,

    UseStmt,
    ExprStmt,

    UseTree,
    UseGroup,

    BinExpr,
    PrefixExpr,
    CallExpr,
    IndexExpr,
    FieldExpr,
    TryExpr,
    GenericExpr,
    BlockExpr,
    LambdaExpr,
    ParenExpr,
    ArrayExpr,
    LitExpr,
    RefExpr,
    SuperExpr,
    IfExpr,
    MatchExpr,
    LoopExpr,
    WhileExpr,
    ForExpr,
    ReturnExpr,
    BreakExpr,
    ContinueExpr,
    ThrowExpr,

    EnumCtor,

    ParamList,
    Param,

    WhereClause,
    ThrowsClause,
    SubtypeSet,

    RetType,

    Generics,
    GenericParam,
}
