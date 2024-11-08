use acvm::FieldElement;
use noirc_errors::{Position, Span, Spanned};
use noirc_frontend::token::IntType;

#[derive(Debug)]
pub(crate) struct SpannedToken(Spanned<Token>);

impl SpannedToken {
    pub(crate) fn new(token: Token, span: Span) -> SpannedToken {
        SpannedToken(Spanned::from(span, token))
    }

    pub(crate) fn to_span(&self) -> Span {
        self.0.span()
    }

    pub(crate) fn token(&self) -> &Token {
        &self.0.contents
    }

    pub(crate) fn into_token(self) -> Token {
        self.0.contents
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum Token {
    Ident(String),
    Int(FieldElement),
    Keyword(Keyword),
    IntType(IntType),
    /// =
    Assign,
    /// (
    LeftParen,
    /// )
    RightParen,
    /// {
    LeftBrace,
    /// }
    RightBrace,
    /// [
    LeftBracket,
    /// ]
    RightBracket,
    /// ,
    Comma,
    /// :
    Colon,
    /// ;
    Semicolon,
    /// ->
    Arrow,
    /// ==
    Equal,
    Eof,
}

impl Token {
    pub(super) fn into_single_span(self, position: Position) -> SpannedToken {
        self.into_span(position, position)
    }

    pub(super) fn into_span(self, start: Position, end: Position) -> SpannedToken {
        SpannedToken(Spanned::from_position(start, end, self))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum Keyword {
    Acir,
    Add,
    And,
    ArrayGet,
    As,
    Bool,
    Brillig,
    Call,
    Cast,
    Constrain,
    Div,
    Inline,
    InlineAlways,
    Else,
    EnableSideEffects,
    Eq,
    Field,
    Fold,
    Fn,
    Index,
    Jmp,
    Jmpif,
    Lt,
    Mod,
    Mul,
    NoPredicates,
    Of,
    Or,
    Return,
    Shl,
    Shr,
    Sub,
    Then,
    Xor,
}

impl Keyword {
    pub(crate) fn lookup_keyword(word: &str) -> Option<Token> {
        let keyword = match word {
            "acir" => Keyword::Acir,
            "add" => Keyword::Add,
            "and" => Keyword::And,
            "array_get" => Keyword::ArrayGet,
            "as" => Keyword::As,
            "bool" => Keyword::Bool,
            "brillig" => Keyword::Brillig,
            "call" => Keyword::Call,
            "cast" => Keyword::Cast,
            "constrain" => Keyword::Constrain,
            "div" => Keyword::Div,
            "else" => Keyword::Else,
            "enable_side_effects" => Keyword::EnableSideEffects,
            "eq" => Keyword::Eq,
            "inline" => Keyword::Inline,
            "inline_always" => Keyword::InlineAlways,
            "Field" => Keyword::Field,
            "fold" => Keyword::Fold,
            "fn" => Keyword::Fn,
            "index" => Keyword::Index,
            "jmp" => Keyword::Jmp,
            "jmpif" => Keyword::Jmpif,
            "lt" => Keyword::Lt,
            "mod" => Keyword::Mod,
            "mul" => Keyword::Mul,
            "no_predicates" => Keyword::NoPredicates,
            "of" => Keyword::Of,
            "or" => Keyword::Or,
            "return" => Keyword::Return,
            "shl" => Keyword::Shl,
            "shr" => Keyword::Shr,
            "sub" => Keyword::Sub,
            "then" => Keyword::Then,
            "xor" => Keyword::Xor,
            _ => return None,
        };
        Some(Token::Keyword(keyword))
    }
}
