use std::ops::Range;

use enum_methods::{EnumIntoGetters, EnumIsA, EnumToGetters};

use crate::model::Spanned;

pub trait Membered: Sized + 'static {
    const ITEMS: &'static [Self];
}

macro_rules! derive_enum {
    // entry point
    ($(#$meta:tt)? $vis:vis enum $name:ident {
        $($xs:ident $(@ $st:tt)?),* $(,)?
    }) => {

        $(#$meta)? $vis enum $name {
            $($xs),*
        }

        impl Membered for $name {
            const ITEMS: &'static [Self] =  &[
                $($name::$xs),*
            ];
        }

        derive_enum!(@fromStr $name {}: $($xs $(@ $st)? ),* ,);
    };

    // fromstr implementation
    (@fromStr $name:ident {$($eout:tt)*}: $member:ident @ $str:tt , $($xs:tt)*) => {
        derive_enum!(@fromStr $name
            { $str => Ok($name::$member), $($eout)*}:
            $($xs)*
        );
    };
    (@fromStr $name:ident {$($eout:tt)*}: $member:ident , $($xs:tt)*) => {
        derive_enum!(@fromStr $name
            { x if x.eq_ignore_ascii_case(stringify!($member))  => Ok($name::$member), $($eout)*}:
            $($xs)*
        );
    };

    (@fromStr $name:ident {$($eout:tt)*}:) => {
        impl std::str::FromStr for $name {
            type Err = ();

            fn from_str(st: &str) -> Result<Self, Self::Err> {
                match st {
                    $($eout)*
                    _ => Err(()),
                }
            }
        }
    };
}

derive_enum!(
    #[derive(Debug, Clone, PartialEq)]
    pub enum SparqlExpr2 {
        Or, Plus @ "+"
    }
);

derive_enum!(
    pub enum SparqlExpr3 {
        Or, Plus @ "+" ,
    }
);

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use super::SparqlExpr2;

    #[test]
    fn test_sparql_expr_2() {
        let or = SparqlExpr2::from_str("or");
        assert_eq!(or, Ok(SparqlExpr2::Or));

        let or = SparqlExpr2::from_str("+");
        assert_eq!(or, Ok(SparqlExpr2::Plus));
    }
}

pub mod semantic_token {
    use lsp_types::SemanticTokenType as STT;
    pub const BOOLEAN: STT = STT::new("boolean");
    pub const LANG_TAG: STT = STT::new("langTag");
}

derive_enum!(
    #[derive(Clone, PartialEq, Eq, Hash, Debug, EnumIntoGetters, EnumIsA, EnumToGetters)]
    pub enum SparqlExpr {
        Or @ "||",
        And @ "&&",
        Equal @ "=",
        NotEqual @ "!=",
        Lt @ "<",
        Gt @ ">",
        Lte @ "<=",
        Gte @ ">=",
        In,
        Not,
        Plus @ "+",
        Minus @ "-",
        Times @ "*",
        Divide @ "/",
        Exclamation @ "!",
    }
);

derive_enum!(
    #[derive(Clone, PartialEq, Eq, Hash, Debug, EnumIntoGetters, EnumIsA, EnumToGetters)]
    pub enum SparqlCall {
        Str,
        Lang,
        LangMatches,
        LangDir,
        Datatype,
        Bound,
        Iri,
        Uri,
        Bnode,
        Rand,
        Abs,
        Ceil,
        Floor,
        Round,
        Concat,
        StrLen,
        Ucase,
        Lcase,
        EncodeForUri,
        Contains,
        StrStarts,
        StrEnds,
        StrBefore,
        StrAfter,
        Year,
        Month,
        Day,
        Hours,
        Minutes,
        Seconds,
        Timezone,
        Tz,
        Now,
        Uuid,
        StrUuid,
        Md5,
        Sha1,
        Sha256,
        Sha384,
        Sha512,
        Coalesce,
        If,
        StrLang,
        StrLangDir,
        StrDt,
        SameTerm,
        IsIri,
        IsUri,
        IsBlank,
        IsLiteral,
        IsNumeric,
        HasLang,
        HasLangDir,
        IsTriple,
        Triple,
        Subject,
        Predicate,
        Object,
    }
);

derive_enum!(
    #[derive(Clone, PartialEq, Eq, Hash, Debug, EnumIntoGetters, EnumIsA, EnumToGetters)]
    pub enum SparqlAggregate {
        Count,
        Sum,
        Min,
        Max,
        Avg,
        Sample,
        GroupConcat,
    }
);

derive_enum!(
    #[derive(Clone, PartialEq, Eq, Hash, Debug, EnumIntoGetters, EnumIsA, EnumToGetters)]
    pub enum SparqlKeyword {
        Regex,
        Substr,
        Replace,
        Exists,
        Select,
        Distinct,
        Reduced,
        Optional,
        Union,
        As,
        Construct,
        Where,
        Describe,
        Ask,
        From,
        Named,
        Group,
        By,
        Having,
        Order,
        Asc,
        Desc,
        Limit,
        Offset,
        Values,
        Load,
        Silent,
        Clear,
        Drop,
        Create,
        Add,
        Move,
        Copy,
        Insert,
        Data,
        Delete,
        With,
        Using,
        Default,
        All,
        Graph,
        Service,
        Bind,
        Undef,
        Minus,
        Filter,
    }
);

#[derive(Clone, PartialEq, Eq, Hash, Debug, EnumIntoGetters, EnumIsA, EnumToGetters)]
pub enum Token {
    /// Sparql expression
    SparqlExpr(SparqlExpr),
    /// Sparql keyword
    SparqlKeyword(SparqlKeyword),
    /// Sparql call
    SparqlCall(SparqlCall),
    /// Sparql aggregate
    SparqlAggregate(SparqlAggregate),
    /// Sparql variable
    Variable(String),
    // Turtle Tokens
    /// @prefix
    PrefixTag,
    /// @base
    BaseTag,
    /// sparql prefix
    SparqlPrefix,
    /// sparql base
    SparqlBase,
    /// a
    PredType,

    /// [
    SqOpen,
    /// ]
    SqClose,
    /// {
    CurlOpen,
    /// }
    CurlClose,
    /// (
    BracketOpen,
    /// )
    BracketClose,

    /// ^^
    DataTypeDelim,

    /// .
    Stop,
    /// ;
    PredicateSplit,
    /// ,
    Comma,

    /// true
    True,
    /// false
    False,
    /// <...>
    IRIRef(String),

    /// ..:
    PNameLN(Option<String>, String),
    /// _:...
    BlankNodeLabel(String),
    /// @...
    LangTag(String),

    Number(String),
    /// All string types
    Str(String, StringStyle),

    /// [ ]
    ANON,
    Comment(String),

    /// :
    Colon,
    /// null
    Null,

    Invalid(String),
}

impl crate::lang::Token for Token {
    fn token(&self) -> Option<lsp_types::SemanticTokenType> {
        match self {
            Token::PrefixTag
            | Token::BaseTag
            | Token::SparqlPrefix
            | Token::SparqlBase
            | Token::PredType
            | Token::SparqlKeyword(_)
            | Token::SparqlCall(_) => Some(lsp_types::SemanticTokenType::KEYWORD),
            Token::True | Token::False => Some(semantic_token::BOOLEAN),
            Token::IRIRef(_) => Some(lsp_types::SemanticTokenType::PROPERTY),
            Token::LangTag(_) => Some(semantic_token::LANG_TAG),
            Token::Number(_) => Some(lsp_types::SemanticTokenType::NUMBER),
            Token::Str(_, _) => Some(lsp_types::SemanticTokenType::STRING),
            Token::Comment(_) => Some(lsp_types::SemanticTokenType::COMMENT),
            Token::Variable(_) => Some(lsp_types::SemanticTokenType::VARIABLE),
             _ => None,
        }
    }

    fn span_tokens(
        Spanned(this, span): &Spanned<Self>,
    ) -> Vec<(lsp_types::SemanticTokenType, Range<usize>)> {
        if let Some(t) = this.token() {
            return vec![(t, span.clone())];
        }

        match this {
            Token::PNameLN(p, _) => {
                let s = p.as_ref().map(|x| x.len()).unwrap_or(0);

                vec![
                    (
                        lsp_types::SemanticTokenType::NAMESPACE,
                        span.start..span.start + 1 + s,
                    ),
                    (
                        lsp_types::SemanticTokenType::ENUM_MEMBER,
                        span.start + s + 1..span.end,
                    ),
                ]
            }
            Token::BlankNodeLabel(_) => {
                vec![
                    (
                        lsp_types::SemanticTokenType::NAMESPACE,
                        span.start..span.start + 2,
                    ),
                    (
                        lsp_types::SemanticTokenType::PROPERTY,
                        span.start + 2..span.end,
                    ),
                ]
            }
            _ => vec![],
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, EnumIntoGetters, EnumIsA, EnumToGetters)]
pub enum StringStyle {
    /// """..."""
    DoubleLong,
    /// "..."
    Double,
    /// '''...'''
    SingleLong,
    /// '...'
    Single,
}

impl StringStyle {
    pub fn quote(&self) -> &'static str {
        match self {
            StringStyle::DoubleLong => "\"\"\"",
            StringStyle::Double => "\"",
            StringStyle::SingleLong => "'''",
            StringStyle::Single => "'",
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::PrefixTag => write!(f, "'@prefix'"),
            Token::BaseTag => write!(f, "'@base'"),
            Token::SparqlPrefix => write!(f, "'PREFIX'"),
            Token::SparqlBase => write!(f, "'BASE'"),
            Token::PredType => write!(f, "'a'"),
            Token::SqOpen => write!(f, "'['"),
            Token::SqClose => write!(f, "']'"),
            Token::BracketOpen => write!(f, "'('"),
            Token::BracketClose => write!(f, "')'"),
            Token::DataTypeDelim => write!(f, "'^^'"),
            Token::Stop => write!(f, "'.'"),
            Token::PredicateSplit => write!(f, "';'"),
            Token::Comma => write!(f, "','"),
            Token::True => write!(f, "'true'"),
            Token::False => write!(f, "'false'"),
            Token::IRIRef(_) => write!(f, "a named node"),
            Token::PNameLN(_, _) => write!(f, "a prefixed node"),
            Token::BlankNodeLabel(_) => write!(f, "a blank node"),
            Token::LangTag(_) => write!(f, "a language tag"),
            Token::Number(_) => write!(f, "a number"),
            Token::Str(_, _) => write!(f, "a string"),
            Token::ANON => write!(f, "an inline blank node"),
            Token::Comment(_) => write!(f, "a comment"),
            Token::Invalid(_) => write!(f, "invalid token"),
            Token::CurlOpen => write!(f, "'{{'"),
            Token::CurlClose => write!(f, "'}}'"),
            Token::Colon => write!(f, "':'"),
            Token::Null => write!(f, "'null'"),
            Token::SparqlExpr(_) => write!(f, "sparql expr token"),
            Token::SparqlKeyword(_) => write!(f, "sparql keyword"),
            Token::SparqlCall(_) => write!(f, "sparql call"),
            Token::SparqlAggregate(_) => write!(f, "sparql aggregate"),
            Token::Variable(_) => write!(f, "sparql variable"),
        }
    }
}
