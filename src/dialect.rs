use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;

/// This is a hack for some statements sqlparser failed to parser.
/// A bad pratice, will be removed if sqlparser support them.
///
/// All illegal statements will be fallback into `Statement::Comment`.
///
/// All legal statements will be fallback into `Statement::ShowVariable`
///
/// As These two will **never** exist in mysql contract.
use sqlparser::ast::CommentObject;
use sqlparser::ast::Ident;
use sqlparser::ast::ObjectName;
use sqlparser::ast::Statement;
use sqlparser::dialect::Dialect;
use sqlparser::keywords::Keyword;
use sqlparser::parser::{Parser, ParserError};
use sqlparser::tokenizer::Token;
use sqlparser::tokenizer::Word;

const DEFAULT_DELIMITER: Token = Token::SemiColon;

#[derive(Debug)]
pub struct MysqlBeyondDialect {
    pub is_recalled: AtomicBool,
}

pub struct FixedStatement(pub Statement);

impl ToString for FixedStatement {
    fn to_string(&self) -> String {
        match &self.0 {
            Statement::Comment {
                object_type: _,
                object_name: _,
                comment,
            } => comment.clone().unwrap_or("".to_string()),
            _ => self.0.to_string(),
        }
    }
}

pub enum AddupStatement {
    Delimiter,
    CreateFunction,
    CreateProcedure,
    CreateTrigger,
    CreateEvent,
    CreateFullText,
    XA,
    Unknown,
    EndEarly,
}

impl From<String> for AddupStatement {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Delimiter" => Self::Delimiter,
            "CreateFunction" => Self::CreateFunction,
            "CreateProcedure" => Self::CreateProcedure,
            "CreateTrigger" => Self::CreateTrigger,
            "CreateEvent" => Self::CreateEvent,
            "CreateFullText" => Self::CreateFullText,
            "XA" => Self::XA,
            "Unknown" => Self::Unknown,
            "EndEarly" => Self::EndEarly,
            _ => unreachable!(),
        }
    }
}

impl ToString for AddupStatement {
    fn to_string(&self) -> String {
        match self {
            AddupStatement::Delimiter => "Delimiter".to_string(),
            AddupStatement::CreateFunction => "CreateFunction".to_string(),
            AddupStatement::CreateProcedure => "CreateProcedure".to_string(),
            AddupStatement::CreateTrigger => "CreateTrigger".to_string(),
            AddupStatement::CreateEvent => "CreateEvent".to_string(),
            AddupStatement::CreateFullText => "CreateFullText".to_string(),
            AddupStatement::XA => "XA".to_string(),
            AddupStatement::Unknown => "Unknown".to_string(),
            AddupStatement::EndEarly => "EndEarly".to_string(),
        }
    }
}

impl MysqlBeyondDialect {
    fn parse_legal(&self, parser: &mut Parser) -> Option<Result<Statement, ParserError>> {
        loop {
            let token = parser.next_token();
            if token == DEFAULT_DELIMITER {
                parser.prev_token();
                break;
            }
            match token {
                Token::EOF => {
                    return Some(Err(ParserError::ParserError("meet EOF".to_owned())));
                }
                _ => continue,
            }
        }
        // fallback to ShowVariable(not really a ShowVariable)
        Some(Ok(Statement::ShowVariable { variable: vec![] }))
    }

    fn parse_delimiter_statement(
        &self,
        parser: &mut Parser,
    ) -> Option<Result<Statement, ParserError>> {
        // TODO: warning cannot detect
        let mut meet_delimiter = false;
        loop {
            let token = parser.next_token();
            match token {
                DEFAULT_DELIMITER => {
                    if meet_delimiter {
                        parser.prev_token();
                        break;
                    }
                }
                Token::EOF => {
                    return Some(Err(ParserError::ParserError("meet EOF".to_owned())));
                }
                Token::Word(w) => {
                    if w.keyword == Keyword::DELIMITER {
                        meet_delimiter = true;
                    }
                }
                _ => continue,
            }
        }
        // fallback to Comment(not really a Comment)
        Some(Ok(Statement::Comment {
            object_type: CommentObject::Table,
            object_name: ObjectName(vec![Ident {
                value: AddupStatement::Delimiter.to_string(),
                quote_style: None,
            }]),
            comment: None,
        }))
    }

    fn parse_illegal(
        &self,
        parser: &mut Parser,
        typ: AddupStatement,
    ) -> Option<Result<Statement, ParserError>> {
        let mut sql = "".to_string();
        loop {
            let token = parser.next_token();
            sql += &token.to_string();
            sql += " ";
            if token == DEFAULT_DELIMITER {
                parser.prev_token();
                break;
            }
            match token {
                Token::EOF => {
                    return Some(Err(ParserError::ParserError("meet EOF".to_owned())));
                }
                _ => continue,
            }
        }
        // fallback to Comment(not really a Comment)
        Some(Ok(Statement::Comment {
            object_type: CommentObject::Table,
            object_name: ObjectName(vec![Ident {
                value: typ.to_string(),
                quote_style: None,
            }]),
            comment: Some(sql),
        }))
    }
}

fn parse_tokens(parser: &mut Parser, tokens: &[Token]) -> bool {
    for (i, wanted_token) in tokens.into_iter().enumerate() {
        match wanted_token {
            Token::Word(w) if w.keyword != Keyword::NoKeyword => {
                if parser.parse_keyword(w.keyword) {
                    continue;
                } else {
                    return false;
                }
            }
            _ => {
                let token = parser.next_token();
                if token != *wanted_token {
                    for _ in 0..i + 1 {
                        parser.prev_token()
                    }
                    return false;
                }
                continue;
            }
        }
    }
    true
}

#[inline]
fn str_to_token(value: String) -> Token {
    Token::Word(Word {
        value,
        quote_style: None,
        keyword: Keyword::NoKeyword,
    })
}

#[inline]
fn key_to_token(keyword: Keyword) -> Token {
    Token::Word(Word {
        value: "".to_string(),
        quote_style: None,
        keyword,
    })
}

impl Dialect for MysqlBeyondDialect {
    fn parse_statement(&self, parser: &mut Parser) -> Option<Result<Statement, ParserError>> {
        if self.is_recalled.load(SeqCst) {
            return None;
        }
        if parser.parse_keyword(Keyword::DELIMITER) {
            return self.parse_delimiter_statement(parser);
        } else if parser.parse_keywords(&[Keyword::CREATE, Keyword::PROCEDURE]) {
            return self.parse_illegal(parser, AddupStatement::CreateProcedure);
        } else if parser.parse_keywords(&[Keyword::CREATE, Keyword::FUNCTION]) {
            return self.parse_illegal(parser, AddupStatement::CreateFunction);
        } else if parser.parse_keywords(&[Keyword::CREATE, Keyword::TRIGGER]) {
            return self.parse_illegal(parser, AddupStatement::CreateTrigger);
        } else if parser.parse_keywords(&[Keyword::CREATE, Keyword::EVENT]) {
            return self.parse_illegal(parser, AddupStatement::CreateEvent);
        } else if parser.parse_keywords(&[Keyword::DROP, Keyword::DATABASE]) {
            return self.parse_legal(parser);
        } else if parse_tokens(
            parser,
            &[
                str_to_token("lock".to_string()),
                key_to_token(Keyword::TABLES),
            ],
        ) {
            return self.parse_legal(parser);
        } else if parse_tokens(
            parser,
            &[
                str_to_token("unlock".to_string()),
                key_to_token(Keyword::TABLES),
            ],
        ) {
            return self.parse_legal(parser);
        } else if parse_tokens(
            parser,
            &[
                key_to_token(Keyword::CREATE),
                str_to_token("fulltext".to_string()),
                key_to_token(Keyword::INDEX),
            ],
        ) {
            return self.parse_illegal(parser, AddupStatement::CreateFullText);
        } else if parse_tokens(parser, &[str_to_token("xa".to_string())]) {
            return self.parse_illegal(parser, AddupStatement::XA);
        }
        self.is_recalled.store(true, SeqCst);
        let original_ans = parser.parse_statement();
        self.is_recalled.store(false, SeqCst);

        match original_ans {
            Ok(s) => match parser.peek_token() {
                Token::SemiColon => Some(Ok(s)),
                _ => return self.parse_illegal(parser, AddupStatement::EndEarly),
            },
            Err(_) => self.parse_illegal(parser, AddupStatement::Unknown),
        }
    }

    fn is_identifier_start(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch)
            || ('A'..='Z').contains(&ch)
            || ch == '_'
            || ch == '$'
            || ch == '@'
            || ('\u{0080}'..='\u{ffff}').contains(&ch)
    }

    fn is_delimited_identifier_start(&self, ch: char) -> bool {
        ch == '"' || ch == '`' || ch == '\''
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        self.is_identifier_start(ch) || ('0'..='9').contains(&ch)
    }
}
