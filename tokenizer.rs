use crate::token::{Token, Keyword};
use std::str::Chars;
use std::iter::Peekable;

pub struct Tokenizer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    //constructor
    //make new tokenizer by turning the input string into a peekable character iterator
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            input: input.chars().peekable(),
        }
    }

    //read characters and returns the next token
    fn next_token(&mut self) -> Token {
        while let Some(&ch) = self.input.peek() {
            match ch {
                //skip whitespace
                ' ' | '\n' | '\t' | '\r' => {
                    self.input.next();
                }

                //single character tokens
                '+' => return self.consume_single(Token::Plus),
                '-' => return self.consume_single(Token::Minus),
                '*' => return self.consume_single(Token::Star),
                '/' => return self.consume_single(Token::Divide),
                '(' => return self.consume_single(Token::LeftParentheses),
                ')' => return self.consume_single(Token::RightParentheses),
                ',' => return self.consume_single(Token::Comma),
                ';' => return self.consume_single(Token::Semicolon),
                '=' => return self.consume_single(Token::Equal),

                //two-character tokens
                '>' => {
                    self.input.next();
                    if self.consume_if('=') {
                        return Token::GreaterThanOrEqual;
                    }
                    return Token::GreaterThan;
                }

                '<' => {
                    self.input.next();
                    if self.consume_if('=') {
                        return Token::LessThanOrEqual;
                    }
                    return Token::LessThan;
                }

                '!' => {
                    self.input.next();
                    if self.consume_if('=') {
                        return Token::NotEqual;
                    }
                    return Token::Invalid('!');
                }

                // String literals
                '"' | '\'' => return self.read_string(),

                // Numbers
                ch if ch.is_ascii_digit() => return self.read_number(),

                // Identifiers or Keywords
                ch if ch.is_ascii_alphabetic() || ch == '_' => return self.read_word(),

                _ => {
                    self.input.next();
                    return Token::Invalid(ch);
                }
            }
        }

        Token::Eof
    }

    //helper, used for simple one-character tokens
    fn consume_single(&mut self, token: Token) -> Token {
        self.input.next();
        token
    }

    //helper, used to check if the next character matches expected
    fn consume_if(&mut self, expected: char) -> bool {
        if self.input.peek() == Some(&expected) {
            self.input.next();
            true
        } else {
            false
        }
    }

    //helper, read a sequence of digits and returns number token
    fn read_number(&mut self) -> Token {
        let mut number = String::new();
        while let Some(&ch) = self.input.peek() {
            if ch.is_ascii_digit() {
                number.push(ch);
                self.input.next();
            } else {
                break;
            }
        }

        Token::Number(number.parse::<u64>().unwrap())
    }

    //helper, reads string enclosed in matching quotes
    fn read_string(&mut self) -> Token {
        let quote = self.input.next().unwrap(); //opening quote
        let mut content = String::new();

        while let Some(&ch) = self.input.peek() {
            if ch == quote {
                self.input.next(); // closing quote
                return Token::String(content);
            } else {
                content.push(ch);
                self.input.next();
            }
        }

        //reached end without closing quote
        Token::Invalid(quote)
    }

    //helper, reads a word consisting of letters/digits/underscores
    fn read_word(&mut self) -> Token {
        let mut word = String::new();
        while let Some(&ch) = self.input.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                word.push(ch);
                self.input.next();
            } else {
                break;
            }
        }

        match word.to_uppercase().as_str() {
            "SELECT" => Token::Keyword(Keyword::Select),
            "FROM" => Token::Keyword(Keyword::From),
            "WHERE" => Token::Keyword(Keyword::Where),
            "CREATE" => Token::Keyword(Keyword::Create),
            "TABLE" => Token::Keyword(Keyword::Table),
            "ORDER" => Token::Keyword(Keyword::Order),
            "BY" => Token::Keyword(Keyword::By),
            "ASC" => Token::Keyword(Keyword::Asc),
            "DESC" => Token::Keyword(Keyword::Desc),
            "AND" => Token::Keyword(Keyword::And),
            "OR" => Token::Keyword(Keyword::Or),
            "NOT" => Token::Keyword(Keyword::Not),
            "TRUE" => Token::Keyword(Keyword::True),
            "FALSE" => Token::Keyword(Keyword::False),
            "PRIMARY" => Token::Keyword(Keyword::Primary),
            "KEY" => Token::Keyword(Keyword::Key),
            "CHECK" => Token::Keyword(Keyword::Check),
            "INT" => Token::Keyword(Keyword::Int),
            "BOOL" => Token::Keyword(Keyword::Bool),
            "VARCHAR" => Token::Keyword(Keyword::Varchar),
            "NULL" => Token::Keyword(Keyword::Null),
            _ => Token::Identifier(word),
        }
    }
}

//making tokenizer an iterator
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        if token == Token::Eof {
            None // signal that iteration is finished
        } else {
            Some(token)
        }
    }
}