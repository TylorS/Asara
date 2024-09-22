use logos::Logos;
use std::ops::Range;

/**
 * TODO:
 * Effect
 * Handlers
 * Match
 * Perform
 * Resume
 * Yield
 */
pub type Span = Range<usize>;

pub fn span(start: usize, end: usize) -> Span {
    Range { start, end }
}

#[derive(Logos, Debug, Clone, Hash, PartialEq, Eq)]
pub enum Token<'a> {
    // Literals
    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral(&'a str),
    #[regex(r"-?[0-9]+", priority = 3)]
    IntegerLiteral(&'a str),
    #[regex(r"-?[0-9]+\.[0-9]+([eE][+-]?[0-9]+)?", priority = 2)]
    DecimalLiteral(&'a str),
    #[regex(r"-?[0-9]+n", priority = 1)]
    BigIntegerLiteral(&'a str),
    #[regex(r"-?[0-9]+(\.[0-9]+)?n", priority = 0)]
    BigDecimalLiteral(&'a str),
    #[regex(r"true|false")]
    BooleanLiteral(&'a str),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier(&'a str),
    #[regex(r"Symbol\([^)]*\)")]
    SymbolLiteral(&'a str),

    // Keywords
    #[token("brand")]
    Brand,
    #[token("continue")]
    Continue,
    #[token("data")]
    Data,
    #[token("effect")]
    Effect,
    #[token("else")]
    Else,
    #[token("export")]
    Export,
    #[token("for")]
    For,
    #[token("function")]
    Function,
    #[token("handle")]
    Handle,
    #[token("if")]
    If,
    #[token("import")]
    Import,
    #[token("in")]
    In,
    #[token("let")]
    Let,
    #[token("match")]
    Match,
    #[token("of")]
    Of,
    #[token("out")]
    Out,
    #[token("perform")]
    Perform,
    #[token("resume")]
    Resume,
    #[token("return")]
    Return,
    #[token("type")]
    Type,
    #[token("while")]
    While,
    #[token("yield")]
    Yield,

    // Type constraints
    #[token("<:")]
    Subtype,
    #[token(">:")]
    Supertype,

    // Operators (sorted by priority)
    #[token("(")]
    LeftParenthesis,
    #[token(")")]
    RightParenthesis,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("...")]
    Spread,
    #[token(".")]
    Dot,
    #[token("::")]
    DoubleColon,
    #[token("?.")]
    OptionalChaining,
    #[token("++")]
    PlusPlus,
    #[token("--")]
    MinusMinus,
    #[token("!")]
    LogicalNot,
    #[token("~")]
    BitwiseNot,
    #[token("**")]
    Exponent,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,
    #[token("%")]
    Modulo,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("<<")]
    LeftShift,
    #[token(">>")]
    RightShift,
    #[token("<")]
    LessThan,
    #[token("<=")]
    LessThanOrEqual,
    #[token(">")]
    GreaterThan,
    #[token(">=")]
    GreaterThanOrEqual,
    #[token("==")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token("&")]
    Ampersand,
    #[token("^")]
    BitwiseXor,
    #[token("|")]
    Pipe,
    #[token("&&")]
    LogicalAnd,
    #[token("||")]
    LogicalOr,
    #[token("??")]
    NullishCoalescing,
    #[token("?")]
    QuestionMark,
    #[token(":")]
    Colon,
    #[token("=")]
    Equals,
    #[token("+=")]
    PlusEquals,
    #[token("-=")]
    MinusEquals,
    #[token("*=")]
    MultiplyEquals,
    #[token("/=")]
    DivideEquals,
    #[token("%=")]
    ModuloEquals,
    #[token("**=")]
    ExponentEquals,
    #[token("<<=")]
    LeftShiftEquals,
    #[token(">>=")]
    RightShiftEquals,
    #[token("&=")]
    AmpersandEquals,
    #[token("^=")]
    BitwiseXorEquals,
    #[token("|=")]
    PipeEquals,
    #[token("&&=")]
    LogicalAndEquals,
    #[token("||=")]
    LogicalOrEquals,
    #[token("??=")]
    NullishCoalescingEquals,
    #[token("->")]
    Arrow,
    #[token("=>")]
    FatArrow,
    #[token("|>")]
    PipeOperator,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token("@")]
    At,
    #[token("#")]
    Hash,

    // Error handling
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

pub struct Lexer<'a> {
    source: &'a str,
    lexer: logos::Lexer<'a, Token<'a>>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token<'a>, Span);

    fn next(&mut self) -> Option<Self::Item> {
      let token = self.lexer.next()?;
      Some((token, self.lexer.span()))
    }
}
