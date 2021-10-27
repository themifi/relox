use super::{
    error::format_error,
    expression::{Binary, Expression, Grouping, Literal, Unary, Variable},
    statement::{ExpressionStatement, Print, Statement, Var},
    token::{Token, TokenType},
};
use std::fmt;

pub fn parse(tokens: Vec<Token>) -> StatementsResult {
    let mut reader = Reader::new(tokens);

    let mut statements = Vec::new();
    while !reader.is_at_end() {
        statements.push(parse_with_reader(&mut reader)?);
    }
    Ok(statements)
}

type StatementsResult = Result<Vec<Box<dyn Statement>>, Error>;
type StatementResult = Result<Box<dyn Statement>, Error>;
type ExpressionResult = Result<Box<dyn Expression>, Error>;

fn parse_with_reader(reader: &mut Reader) -> StatementResult {
    let result = declaration(reader);
    if result.is_err() {
        syncronize(reader);
    }
    result
}

fn declaration(reader: &mut Reader) -> StatementResult {
    if let Some(TokenType::Var) = reader.peek_type() {
        reader.advance();
        var_declaration(reader)
    } else {
        statement(reader)
    }
}

fn var_declaration(reader: &mut Reader) -> StatementResult {
    let name = reader.advance();
    if name.as_ref().map(|x| x.t) != Some(TokenType::Identifier) {
        return Err(Error::ExpectVariableName {
            line: reader.line(),
        });
    }
    let name = name.unwrap();

    let initializer = if reader.peek_type() == Some(TokenType::Equal) {
        reader.advance().unwrap();
        Some(expression(reader)?)
    } else {
        None
    };

    let token_type = reader.advance().map(|x| x.t);
    if token_type != Some(TokenType::Semicolon) {
        return Err(Error::ExpectSemicolonAfterVariableDeclaration {
            line: reader.line(),
        });
    }
    Ok(Box::new(Var { name, initializer }))
}

fn statement(reader: &mut Reader) -> StatementResult {
    if let Some(TokenType::Print) = reader.peek_type() {
        reader.advance();
        print_statement(reader)
    } else {
        expression_statement(reader)
    }
}

fn print_statement(reader: &mut Reader) -> StatementResult {
    let expr = expression(reader)?;
    let token_type = reader.advance().map(|x| x.t);
    if token_type != Some(TokenType::Semicolon) {
        return Err(Error::ExpectSemicolonAfterValue {
            line: reader.line(),
        });
    }
    Ok(Box::new(Print { expr }))
}

fn expression_statement(reader: &mut Reader) -> StatementResult {
    let expr = expression(reader)?;
    let token_type = reader.advance().map(|x| x.t);
    if token_type != Some(TokenType::Semicolon) {
        return Err(Error::ExpectSemicolonAfterExpression {
            line: reader.line(),
        });
    }
    Ok(Box::new(ExpressionStatement { expr }))
}

fn expression(reader: &mut Reader) -> ExpressionResult {
    equality(reader)
}

fn equality(reader: &mut Reader) -> ExpressionResult {
    let mut expr = comparsion(reader)?;

    while let Some(TokenType::BangEqual) | Some(TokenType::EqualEqual) = reader.peek_type() {
        let operator = reader.advance().unwrap();
        let right = comparsion(reader)?;
        expr = Box::new(Binary {
            left: expr,
            operator,
            right,
        });
    }

    Ok(expr)
}

fn comparsion(reader: &mut Reader) -> ExpressionResult {
    let mut expr = term(reader)?;

    while let Some(TokenType::Greater)
    | Some(TokenType::GreaterEqual)
    | Some(TokenType::Less)
    | Some(TokenType::LessEqual) = reader.peek_type()
    {
        let operator = reader.advance().unwrap();
        let right = term(reader)?;
        expr = Box::new(Binary {
            left: expr,
            operator,
            right,
        });
    }

    Ok(expr)
}

fn term(reader: &mut Reader) -> ExpressionResult {
    let mut expr = factor(reader)?;

    while let Some(TokenType::Minus) | Some(TokenType::Plus) = reader.peek_type() {
        let operator = reader.advance().unwrap();
        let right = factor(reader)?;
        expr = Box::new(Binary {
            left: expr,
            operator,
            right,
        });
    }

    Ok(expr)
}

fn factor(reader: &mut Reader) -> ExpressionResult {
    let mut expr = unary(reader)?;

    while let Some(TokenType::Slash) | Some(TokenType::Star) = reader.peek_type() {
        let operator = reader.advance().unwrap();
        let right = unary(reader)?;
        expr = Box::new(Binary {
            left: expr,
            operator,
            right,
        });
    }

    Ok(expr)
}

fn unary(reader: &mut Reader) -> ExpressionResult {
    match reader.peek_type() {
        Some(TokenType::Bang) | Some(TokenType::Minus) => {
            let operator = reader.advance().unwrap();
            let right = unary(reader)?;
            let expr = Box::new(Unary { operator, right });
            Ok(expr)
        }
        _ => primary(reader),
    }
}

fn primary(reader: &mut Reader) -> ExpressionResult {
    match reader.peek_type() {
        Some(TokenType::True)
        | Some(TokenType::False)
        | Some(TokenType::Nil)
        | Some(TokenType::Number)
        | Some(TokenType::String) => {
            let token = reader.advance().unwrap();
            let expr = Box::new(Literal {
                value: token.literal.unwrap(),
            });
            Ok(expr)
        }
        Some(TokenType::LeftParen) => {
            reader.advance();
            let expr = expression(reader)?;
            let token_type = reader.advance().map(|x| x.t);
            if token_type != Some(TokenType::RightParen) {
                return Err(Error::RightParenExpected {
                    line: reader.line(),
                });
            }
            Ok(Box::new(Grouping { expr }))
        }
        Some(TokenType::Identifier) => {
            let name = reader.advance().unwrap();
            Ok(Box::new(Variable { name }))
        }
        None => Err(Error::ExpressionExpected {
            line: reader.line(),
        }),
        _ => {
            let token = reader.advance().unwrap();
            Err(Error::UnexpectedToken {
                line: token.line,
                lexeme: token.lexeme,
            })
        }
    }
}

fn syncronize(reader: &mut Reader) {
    loop {
        match reader.peek_type() {
            Some(TokenType::Semicolon) => {
                reader.advance();
                return;
            }
            Some(TokenType::Class)
            | Some(TokenType::Fun)
            | Some(TokenType::Var)
            | Some(TokenType::For)
            | Some(TokenType::If)
            | Some(TokenType::While)
            | Some(TokenType::Print)
            | Some(TokenType::Return)
            | None => break,
            _ => reader.advance(),
        };
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    RightParenExpected { line: usize },
    UnexpectedToken { line: usize, lexeme: String },
    ExpressionExpected { line: usize },
    ExpectSemicolonAfterValue { line: usize },
    ExpectSemicolonAfterExpression { line: usize },
    ExpectSemicolonAfterVariableDeclaration { line: usize },
    ExpectVariableName { line: usize },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Self::RightParenExpected { line } => format_error(line, "expect ')' after expression"),
            Self::UnexpectedToken { line, ref lexeme } => {
                format_error(line, format!("unexpected token: {:?}", lexeme))
            }
            Self::ExpressionExpected { line } => format_error(line, "expression expected"),
            Self::ExpectSemicolonAfterValue { line } => {
                format_error(line, "expect ';' after value")
            }
            Self::ExpectSemicolonAfterExpression { line } => {
                format_error(line, "expect ';' after expression")
            }
            Self::ExpectSemicolonAfterVariableDeclaration { line } => {
                format_error(line, "expect ';' after variable declaration")
            }
            Self::ExpectVariableName { line } => format_error(line, "expect ';' variable name"),
        };
        write!(f, "{}", msg)
    }
}

struct Reader {
    iter: std::vec::IntoIter<Token>,
    current: Option<Token>,
    last_line: usize,
}

impl Reader {
    fn new(tokens: Vec<Token>) -> Self {
        let mut iter = tokens.into_iter();
        let current = iter.next();
        let last_line = current.as_ref().unwrap().line;
        Self {
            last_line,
            iter,
            current,
        }
    }

    fn peek_type(&self) -> Option<TokenType> {
        self.current.as_ref().map(|x| x.t)
    }

    fn advance(&mut self) -> Option<Token> {
        let mut next = self.iter.next();

        if let Some(token) = &self.current {
            self.last_line = token.line;
        }

        std::mem::swap(&mut self.current, &mut next);
        next
    }

    fn line(&self) -> usize {
        self.last_line
    }

    fn is_at_end(&self) -> bool {
        self.peek_type() == Some(TokenType::Eof)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::token::{Literal as TokenLiteral, *},
        *,
    };

    fn parse_expression(tokens: Vec<Token>) -> ExpressionResult {
        let mut reader = Reader::new(tokens);
        expression(&mut reader)
    }

    #[test]
    fn test_parse_literals_true() {
        let tokens = vec![Token {
            t: TokenType::True,
            lexeme: "true".to_owned(),
            literal: Some(TokenLiteral::Boolean(true)),
            line: 1,
        }];

        let tree = parse_expression(tokens).unwrap();

        assert_eq!("true", format!("{}", tree));
    }

    #[test]
    fn test_parse_literals_false() {
        let tokens = vec![Token {
            t: TokenType::False,
            lexeme: "false".to_owned(),
            literal: Some(TokenLiteral::Boolean(false)),
            line: 1,
        }];

        let tree = parse_expression(tokens).unwrap();

        assert_eq!("false", format!("{}", tree));
    }

    #[test]
    fn test_parse_literals_nil() {
        let tokens = vec![Token {
            t: TokenType::Nil,
            lexeme: "nil".to_owned(),
            literal: Some(TokenLiteral::Nil),
            line: 1,
        }];

        let tree = parse_expression(tokens).unwrap();

        assert_eq!("nil", format!("{}", tree));
    }

    #[test]
    fn test_parse_literals_string() {
        let tokens = vec![Token {
            t: TokenType::String,
            lexeme: "foo".to_owned(),
            literal: Some(TokenLiteral::String("foo".to_owned())),
            line: 1,
        }];

        let tree = parse_expression(tokens).unwrap();

        assert_eq!("\"foo\"", format!("{}", tree));
    }

    #[test]
    fn test_parse_literals_number() {
        let tokens = vec![Token {
            t: TokenType::Number,
            lexeme: "3.15".to_owned(),
            literal: Some(TokenLiteral::Number(3.15)),
            line: 1,
        }];

        let tree = parse_expression(tokens).unwrap();

        assert_eq!("3.15", format!("{}", tree));
    }

    #[test]
    fn test_primary_grouping() {
        let tokens = vec![
            Token {
                t: TokenType::LeftParen,
                lexeme: String::new(),
                literal: None,
                line: 1,
            },
            Token {
                t: TokenType::Number,
                lexeme: String::new(),
                literal: Some(TokenLiteral::Number(2.0)),
                line: 1,
            },
            Token {
                t: TokenType::RightParen,
                lexeme: String::new(),
                literal: None,
                line: 1,
            },
        ];

        let tree = parse_expression(tokens).unwrap();

        assert_eq!("(group 2)", format!("{}", tree));
    }

    #[test]
    fn test_unary_number() {
        let tokens = vec![
            Token {
                t: TokenType::Minus,
                lexeme: String::new(),
                literal: None,
                line: 1,
            },
            Token {
                t: TokenType::Number,
                lexeme: String::new(),
                literal: Some(TokenLiteral::Number(123.0)),
                line: 1,
            },
        ];

        let tree = parse_expression(tokens).unwrap();

        assert_eq!("(- 123)", format!("{}", tree));
    }

    #[test]
    fn test_unary_boolean() {
        let tokens = vec![
            Token {
                t: TokenType::Bang,
                lexeme: String::new(),
                literal: None,
                line: 1,
            },
            Token {
                t: TokenType::True,
                lexeme: String::new(),
                literal: Some(TokenLiteral::Boolean(true)),
                line: 1,
            },
        ];

        let tree = parse_expression(tokens).unwrap();

        assert_eq!("(! true)", format!("{}", tree));
    }

    #[test]
    fn test_binary() {
        let operators = vec![
            TokenType::Star,
            TokenType::Slash,
            TokenType::BangEqual,
            TokenType::EqualEqual,
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];

        for t in operators {
            let tokens = vec![
                Token {
                    t: TokenType::Number,
                    lexeme: String::new(),
                    literal: Some(TokenLiteral::Number(4.0)),
                    line: 1,
                },
                Token {
                    t,
                    lexeme: String::new(),
                    literal: None,
                    line: 1,
                },
                Token {
                    t: TokenType::Number,
                    lexeme: String::new(),
                    literal: Some(TokenLiteral::Number(2.0)),
                    line: 1,
                },
            ];

            let tree = parse_expression(tokens).unwrap();

            assert_eq!(format!("({} 4 2)", t), format!("{}", tree));
        }
    }

    #[test]
    fn test_factor_unary() {
        let tokens = vec![
            Token {
                t: TokenType::Number,
                lexeme: String::new(),
                literal: Some(TokenLiteral::Number(4.0)),
                line: 1,
            },
            Token {
                t: TokenType::Star,
                lexeme: String::new(),
                literal: None,
                line: 1,
            },
            Token {
                t: TokenType::Minus,
                lexeme: String::new(),
                literal: None,
                line: 1,
            },
            Token {
                t: TokenType::Number,
                lexeme: String::new(),
                literal: Some(TokenLiteral::Number(2.0)),
                line: 1,
            },
        ];

        let tree = parse_expression(tokens).unwrap();

        assert_eq!("(* 4 (- 2))", format!("{}", tree));
    }

    #[test]
    fn test_term_factor() {
        let tokens = vec![
            Token {
                t: TokenType::Number,
                lexeme: String::new(),
                literal: Some(TokenLiteral::Number(5.0)),
                line: 1,
            },
            Token {
                t: TokenType::Plus,
                lexeme: String::new(),
                literal: None,
                line: 1,
            },
            Token {
                t: TokenType::Number,
                lexeme: String::new(),
                literal: Some(TokenLiteral::Number(4.0)),
                line: 1,
            },
            Token {
                t: TokenType::Star,
                lexeme: String::new(),
                literal: None,
                line: 1,
            },
            Token {
                t: TokenType::Number,
                lexeme: String::new(),
                literal: Some(TokenLiteral::Number(2.0)),
                line: 1,
            },
        ];

        let tree = parse_expression(tokens).unwrap();

        assert_eq!("(+ 5 (* 4 2))", format!("{}", tree));
    }

    #[test]
    fn test_comparsion_term() {
        let tokens = vec![
            Token {
                t: TokenType::Number,
                lexeme: String::new(),
                literal: Some(TokenLiteral::Number(5.0)),
                line: 1,
            },
            Token {
                t: TokenType::Greater,
                lexeme: String::new(),
                literal: None,
                line: 1,
            },
            Token {
                t: TokenType::Number,
                lexeme: String::new(),
                literal: Some(TokenLiteral::Number(4.0)),
                line: 1,
            },
            Token {
                t: TokenType::Plus,
                lexeme: String::new(),
                literal: None,
                line: 1,
            },
            Token {
                t: TokenType::Number,
                lexeme: String::new(),
                literal: Some(TokenLiteral::Number(2.0)),
                line: 1,
            },
        ];

        let tree = parse_expression(tokens).unwrap();

        assert_eq!("(> 5 (+ 4 2))", format!("{}", tree));
    }

    #[test]
    fn test_right_paren_expected() {
        let tokens = vec![
            Token {
                t: TokenType::LeftParen,
                lexeme: String::new(),
                literal: None,
                line: 2,
            },
            Token {
                t: TokenType::Number,
                lexeme: String::new(),
                literal: Some(TokenLiteral::Number(3.0)),
                line: 3,
            },
        ];

        let err = parse(tokens).unwrap_err();
        assert_eq!(Error::RightParenExpected { line: 3 }, err);
    }

    #[test]
    fn test_term_token_expected() {
        let tokens = vec![
            Token {
                t: TokenType::Number,
                lexeme: String::new(),
                literal: Some(TokenLiteral::Number(2.0)),
                line: 2,
            },
            Token {
                t: TokenType::Plus,
                lexeme: String::new(),
                literal: None,
                line: 3,
            },
        ];

        let err = parse(tokens).unwrap_err();
        assert_eq!(Error::ExpressionExpected { line: 3 }, err);
    }

    #[test]
    fn test_token_unexpected() {
        let tokens = vec![Token {
            t: TokenType::Plus,
            lexeme: "+".to_owned(),
            literal: None,
            line: 3,
        }];

        let err = parse(tokens).unwrap_err();
        assert_eq!(
            Error::UnexpectedToken {
                line: 3,
                lexeme: "+".to_owned()
            },
            err
        );
    }

    #[test]
    fn test_equality_comparsion() {
        let tokens = vec![
            Token {
                t: TokenType::Number,
                lexeme: String::new(),
                literal: Some(TokenLiteral::Number(5.0)),
                line: 1,
            },
            Token {
                t: TokenType::EqualEqual,
                lexeme: String::new(),
                literal: None,
                line: 1,
            },
            Token {
                t: TokenType::Number,
                lexeme: String::new(),
                literal: Some(TokenLiteral::Number(4.0)),
                line: 1,
            },
            Token {
                t: TokenType::Greater,
                lexeme: String::new(),
                literal: None,
                line: 1,
            },
            Token {
                t: TokenType::Number,
                lexeme: String::new(),
                literal: Some(TokenLiteral::Number(2.0)),
                line: 1,
            },
        ];

        let tree = parse_expression(tokens).unwrap();

        assert_eq!("(== 5 (> 4 2))", format!("{}", tree));
    }

    #[test]
    fn test_reader() {
        let first = Token {
            t: TokenType::Number,
            lexeme: String::new(),
            literal: Some(TokenLiteral::Number(5.0)),
            line: 1,
        };
        let second = Token {
            t: TokenType::EqualEqual,
            lexeme: String::new(),
            literal: None,
            line: 2,
        };
        let third = Token {
            t: TokenType::Nil,
            lexeme: String::new(),
            literal: None,
            line: 3,
        };
        let tokens = vec![first.clone(), second.clone(), third.clone()];

        let mut reader = Reader::new(tokens);

        assert_eq!(1, reader.line());
        assert_eq!(Some(first.t), reader.peek_type());
        assert_eq!(Some(first), reader.advance());

        assert_eq!(1, reader.line());
        assert_eq!(Some(second.t), reader.peek_type());
        assert_eq!(Some(second), reader.advance());

        assert_eq!(2, reader.line());
        assert_eq!(Some(third.t), reader.peek_type());
        assert_eq!(Some(third), reader.advance());

        assert_eq!(3, reader.line());
        assert_eq!(None, reader.peek_type());
        assert_eq!(None, reader.advance());
    }

    #[test]
    fn test_syncronize_on_error_with_semicolon() {
        let stop_token = Token {
            t: TokenType::Number,
            lexeme: String::new(),
            literal: None,
            line: 3,
        };
        let tokens = vec![
            Token {
                t: TokenType::Plus,
                lexeme: "+".to_owned(),
                literal: None,
                line: 3,
            },
            Token {
                t: TokenType::Number,
                lexeme: String::new(),
                literal: None,
                line: 3,
            },
            Token {
                t: TokenType::Semicolon,
                lexeme: String::new(),
                literal: None,
                line: 3,
            },
            stop_token.clone(),
        ];
        let mut reader = Reader::new(tokens);

        let res = parse_with_reader(&mut reader);

        assert!(res.is_err());
        assert_eq!(Some(stop_token), reader.advance());
    }

    #[test]
    fn test_syncronize_on_error_with_fun() {
        let stop_token = Token {
            t: TokenType::Fun,
            lexeme: String::new(),
            literal: None,
            line: 3,
        };
        let tokens = vec![
            Token {
                t: TokenType::Plus,
                lexeme: "+".to_owned(),
                literal: None,
                line: 3,
            },
            Token {
                t: TokenType::Number,
                lexeme: String::new(),
                literal: None,
                line: 3,
            },
            stop_token.clone(),
        ];
        let mut reader = Reader::new(tokens);

        let res = parse_with_reader(&mut reader);

        assert!(res.is_err());
        assert_eq!(Some(stop_token), reader.advance());
    }

    #[test]
    fn test_error_format() {
        assert_eq!(
            "[line 3] Error: expect ')' after expression",
            format!("{}", Error::RightParenExpected { line: 3 })
        );
        assert_eq!(
            "[line 3] Error: unexpected token: \"foo\"",
            format!(
                "{}",
                Error::UnexpectedToken {
                    line: 3,
                    lexeme: "foo".to_owned()
                }
            )
        );
        assert_eq!(
            "[line 3] Error: expression expected",
            format!("{}", Error::ExpressionExpected { line: 3 })
        );
        assert_eq!(
            "[line 3] Error: expect ';' after expression",
            format!("{}", Error::ExpectSemicolonAfterExpression { line: 3 })
        );
        assert_eq!(
            "[line 3] Error: expect ';' after value",
            format!("{}", Error::ExpectSemicolonAfterValue { line: 3 })
        );
    }

    #[test]
    fn parse_till_end() {
        let tokens = vec![
            Token {
                t: TokenType::Number,
                lexeme: String::new(),
                literal: Some(TokenLiteral::Number(2.0)),
                line: 1,
            },
            Token {
                t: TokenType::Semicolon,
                lexeme: String::new(),
                literal: None,
                line: 1,
            },
            Token {
                t: TokenType::Number,
                lexeme: String::new(),
                literal: Some(TokenLiteral::Number(2.0)),
                line: 2,
            },
            Token {
                t: TokenType::Semicolon,
                lexeme: String::new(),
                literal: None,
                line: 2,
            },
            Token {
                t: TokenType::Eof,
                lexeme: String::new(),
                literal: None,
                line: 2,
            },
        ];
        let res = parse(tokens).unwrap();
        assert_eq!(2, res.len());
    }

    fn parse_statement(tokens: Vec<Token>) -> StatementResult {
        let mut reader = Reader::new(tokens);
        declaration(&mut reader)
    }

    #[test]
    fn parse_print_statement() {
        let tokens = vec![
            Token {
                t: TokenType::Print,
                lexeme: String::new(),
                literal: None,
                line: 1,
            },
            Token {
                t: TokenType::Number,
                lexeme: String::new(),
                literal: Some(TokenLiteral::Number(2.0)),
                line: 1,
            },
            Token {
                t: TokenType::Semicolon,
                lexeme: String::new(),
                literal: None,
                line: 1,
            },
        ];

        let tree = parse_statement(tokens).unwrap();

        assert_eq!("(print statement)", format!("{}", tree));
    }

    #[test]
    fn parse_expression_statement() {
        let tokens = vec![
            Token {
                t: TokenType::Number,
                lexeme: String::new(),
                literal: Some(TokenLiteral::Number(2.0)),
                line: 1,
            },
            Token {
                t: TokenType::Semicolon,
                lexeme: String::new(),
                literal: None,
                line: 1,
            },
        ];

        let tree = parse_statement(tokens).unwrap();

        assert_eq!("(expression statement)", format!("{}", tree));
    }

    #[test]
    fn parse_expression_statement_without_semicolon() {
        let tokens = vec![Token {
            t: TokenType::Number,
            lexeme: String::new(),
            literal: Some(TokenLiteral::Number(2.0)),
            line: 1,
        }];

        assert_eq!(
            Error::ExpectSemicolonAfterExpression { line: 1 },
            parse_statement(tokens).unwrap_err()
        );
    }

    #[test]
    fn parse_expression_without_semicolon() {
        let tokens = vec![
            Token {
                t: TokenType::Print,
                lexeme: String::new(),
                literal: None,
                line: 1,
            },
            Token {
                t: TokenType::Number,
                lexeme: String::new(),
                literal: Some(TokenLiteral::Number(2.0)),
                line: 1,
            },
        ];

        assert_eq!(
            Error::ExpectSemicolonAfterValue { line: 1 },
            parse_statement(tokens).unwrap_err()
        );
    }

    #[test]
    fn parse_var_declaration_without_initializer() {
        let tokens = vec![
            Token {
                t: TokenType::Var,
                lexeme: String::new(),
                literal: None,
                line: 1,
            },
            Token {
                t: TokenType::Identifier,
                lexeme: "foo".to_owned(),
                literal: Some(TokenLiteral::Identifier("foo".to_owned())),
                line: 1,
            },
            Token {
                t: TokenType::Semicolon,
                lexeme: String::new(),
                literal: None,
                line: 1,
            },
        ];

        let tree = parse_statement(tokens).unwrap();
        assert_eq!("(var foo)", format!("{}", tree));
    }

    #[test]
    fn parse_var_declaration_with_initializer() {
        let tokens = vec![
            Token {
                t: TokenType::Var,
                lexeme: String::new(),
                literal: None,
                line: 1,
            },
            Token {
                t: TokenType::Identifier,
                lexeme: "foo".to_owned(),
                literal: Some(TokenLiteral::Identifier("foo".to_owned())),
                line: 1,
            },
            Token {
                t: TokenType::Equal,
                lexeme: String::new(),
                literal: None,
                line: 1,
            },
            Token {
                t: TokenType::Number,
                lexeme: String::new(),
                literal: Some(TokenLiteral::Number(2.0)),
                line: 1,
            },
            Token {
                t: TokenType::Semicolon,
                lexeme: String::new(),
                literal: None,
                line: 1,
            },
        ];

        let tree = parse_statement(tokens).unwrap();
        assert_eq!("(var foo = 2)", format!("{}", tree));
    }

    #[test]
    fn parse_identifier() {
        let tokens = vec![Token {
            t: TokenType::Identifier,
            lexeme: "foo".to_owned(),
            literal: Some(TokenLiteral::Identifier("foo".to_owned())),
            line: 1,
        }];

        let tree = parse_expression(tokens).unwrap();

        assert_eq!("(var foo)", format!("{}", tree));
    }
}
