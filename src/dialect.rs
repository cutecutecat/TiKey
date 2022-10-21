/// This is a hack for sqlparser failed to parser
use sqlparser::ast::CommentObject;
use sqlparser::ast::Ident;
use sqlparser::ast::ObjectName;
use sqlparser::ast::Statement;
use sqlparser::dialect::Dialect;
use sqlparser::keywords::Keyword;
use sqlparser::parser::{Parser, ParserError};
use sqlparser::tokenizer::Token;

const DEFAULT_DELIMITER: Token = Token::SemiColon;

#[derive(Debug)]
pub struct MysqlBeyondDialect {}

pub struct FixedStatement(pub Statement);

impl ToString for FixedStatement {
    fn to_string(&self) -> String {
        match &self.0 {
            Statement::Comment {
                object_type: _,
                object_name: _,
                comment,
            } => comment.clone().unwrap(),
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
}

impl From<String> for AddupStatement {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Delimiter" => Self::Delimiter,
            "CreateFunction" => Self::CreateFunction,
            "CreateProcedure" => Self::CreateProcedure,
            "CreateTrigger" => Self::CreateTrigger,
            "CreateEvent" => Self::CreateEvent,
            _ => unreachable!(),
        }
    }
}

impl ToString for AddupStatement {
    fn to_string(&self) -> String {
        match self {
            Self::Delimiter => "Delimiter".to_string(),
            AddupStatement::CreateFunction => "CreateFunction".to_string(),
            AddupStatement::CreateProcedure => "CreateProcedure".to_string(),
            AddupStatement::CreateTrigger => "CreateTrigger".to_string(),
            AddupStatement::CreateEvent => "CreateEvent".to_string(),
        }
    }
}

impl MysqlBeyondDialect {
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
        Some(Ok(Statement::Comment {
            object_type: CommentObject::Table,
            object_name: ObjectName(vec![Ident {
                value: AddupStatement::Delimiter.to_string(),
                quote_style: None,
            }]),
            comment: None,
        }))
    }
    fn parse_addup_statement(
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
        // fallback to comment(not really a comment)
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

impl Dialect for MysqlBeyondDialect {
    fn parse_statement(&self, parser: &mut Parser) -> Option<Result<Statement, ParserError>> {
        if parser.parse_keyword(Keyword::DELIMITER) {
            self.parse_delimiter_statement(parser)
        } else if parser.parse_keywords(&[Keyword::CREATE, Keyword::PROCEDURE]) {
            self.parse_addup_statement(parser, AddupStatement::CreateProcedure)
        } else if parser.parse_keywords(&[Keyword::CREATE, Keyword::FUNCTION]) {
            self.parse_addup_statement(parser, AddupStatement::CreateFunction)
        } else if parser.parse_keywords(&[Keyword::CREATE, Keyword::TRIGGER]) {
            self.parse_addup_statement(parser, AddupStatement::CreateTrigger)
        } else if parser.parse_keywords(&[Keyword::CREATE, Keyword::EVENT]) {
            self.parse_addup_statement(parser, AddupStatement::CreateEvent)
        } else if parser.parse_keywords(&[Keyword::CREATE, Keyword::FULL]) {
            self.parse_addup_statement(parser, AddupStatement::CreateEvent)
        } else {
            None
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

    fn is_identifier_part(&self, ch: char) -> bool {
        self.is_identifier_start(ch) || ('0'..='9').contains(&ch)
    }
}
