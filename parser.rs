use crate::token::{Token, Keyword};
use crate::statement::{
    Statement,
    Expression,
    BinaryOperator,
    UnaryOperator,
    TableColumn,
    DBType,
    Constraint,
};

//holds a list of tokens and a position index for parsing them
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}
//make new parser with token list
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    //peek at current token without going forward
    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    //get current token and move to next
    fn next(&mut self) -> Token {
        let tok = self.tokens[self.pos].clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        tok
    }

    //expect specific token, if it doesnt match, show error
    fn expect(&mut self, expected: &Token) -> Result<(), String>
    where
        Token: PartialEq + std::fmt::Debug,
    {
        if self.peek() == expected {
            self.next();
            Ok(())
        } else {
            Err(format!("Expected {:?}, found {:?}", expected, self.peek()))
        }
    }

    //main entry
    //decide what kind of sql statement to parse
    pub fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.peek() {
            Token::Keyword(Keyword::Select) => {
                self.next();
                self.parse_select()
            }
            Token::Keyword(Keyword::Create) => {
                self.next();
                self.parse_create_table()
            }
            other => Err(format!("Expected SELECT or CREATE, found {:?}", other)),
        }
    }

    //select parsing
    fn parse_select(&mut self) -> Result<Statement, String> {
        //start columns
        let mut columns = Vec::new();
        loop {
            let expr = self.parse_expression(0)?;
            columns.push(expr);
            if let Token::Comma = self.peek() {
                self.next();
                continue;
            }
            break;
        }

        //make sure 'FROM' appears after the SELECT columns
        self.expect(&Token::Keyword(Keyword::From))?;
        let table_name = match self.next() {
            Token::Identifier(s) => s,
            other => return Err(format!("Expected table name, found {:?}", other)),
        };

        //optional WHERE exp
        let where_clause = if let Token::Keyword(Keyword::Where) = self.peek() {
            self.next();
            Some(self.parse_expression(0)?)
        } else {
            None
        };

        //optional ORDER BY exp
        let mut orderby = Vec::new();
        if let Token::Keyword(Keyword::Order) = self.peek() {
            self.next();
            self.expect(&Token::Keyword(Keyword::By))?;
            loop {
                let expr = self.parse_expression(0)?;
                orderby.push(expr);
                if let Token::Comma = self.peek() {
                    self.next();
                    continue;
                }
                break;
            }
        }
        
        self.expect(&Token::Semicolon)?;

        Ok(Statement::Select {
            columns,
            from: table_name,
            r#where: where_clause,
            orderby,
        })
    }

    //create table parsing
    fn parse_create_table(&mut self) -> Result<Statement, String> {
        //confirm TABLE appears after CREATE
        self.expect(&Token::Keyword(Keyword::Table))?;

        //table name
        let table_name = match self.next() {
            Token::Identifier(s) => s,
            other => return Err(format!("Expected table name, found {:?}", other)),
        };
        
        self.expect(&Token::LeftParentheses)?;

        let mut columns = Vec::new();
        loop {
            //end of list?
            if let Token::RightParentheses = self.peek() {
                self.next();
                break;
            }

            //column name
            let col_name = match self.next() {
                Token::Identifier(s) => s,
                other => return Err(format!("Expected column name, found {:?}", other)),
            };

            //column type
            let col_type = match self.peek() {
                Token::Keyword(Keyword::Int) => {
                    self.next();
                    DBType::Int
                }
                Token::Keyword(Keyword::Bool) => {
                    self.next();
                    DBType::Bool
                }
                Token::Keyword(Keyword::Varchar) => {
                    self.next();
                    self.expect(&Token::LeftParentheses)?;
                    let len = match self.next() {
                        Token::Number(n) => n as usize,
                        other => return Err(format!("Expected VARCHAR length, found {:?}", other)),
                    };
                    self.expect(&Token::RightParentheses)?;
                    DBType::Varchar(len)
                }
                other => return Err(format!("Expected type, found {:?}", other)),
            };

            //optional constraints
            let mut constraints = Vec::new();
            loop {
                match self.peek() {
                    Token::Keyword(Keyword::Primary) => {
                        self.next();
                        self.expect(&Token::Keyword(Keyword::Key))?;
                        constraints.push(Constraint::PrimaryKey);
                    }
                    Token::Keyword(Keyword::Not) => {
                        self.next();
                        self.expect(&Token::Keyword(Keyword::Null))?;
                        constraints.push(Constraint::NotNull);
                    }
                    Token::Keyword(Keyword::Check) => {
                        self.next();
                        self.expect(&Token::LeftParentheses)?;
                        let expr = self.parse_expression(0)?;
                        self.expect(&Token::RightParentheses)?;
                        constraints.push(Constraint::Check(expr));
                    }
                    _ => break,
                }
            }

            columns.push(TableColumn {
                column_name: col_name,
                column_type: col_type,
                constraints,
            });

            //comma or end
            match self.peek() {
                Token::Comma => { self.next(); }
                Token::RightParentheses => { self.next(); break; }
                other => return Err(format!("Expected ',' or ')', found {:?}", other)),
            }
        }
        
        self.expect(&Token::Semicolon)?;

        Ok(Statement::CreateTable {
            table_name,
            column_list: columns,
        })
    }

    //pratt parsing for expressions
    fn parse_expression(&mut self, min_prec: u8) -> Result<Expression, String> {
        //parse prefix
        let mut left = match self.next() {
            Token::Number(n) => Expression::Number(n),
            Token::Identifier(s) => Expression::Identifier(s),
            Token::String(s) => Expression::String(s),
            Token::Keyword(Keyword::True) => Expression::Bool(true),
            Token::Keyword(Keyword::False) => Expression::Bool(false),
            Token::LeftParentheses => {
                let expr = self.parse_expression(0)?;
                self.expect(&Token::RightParentheses)?;
                expr
            }
            Token::Minus => {
                let rhs = self.parse_expression(100)?;
                Expression::UnaryOperation { operand: Box::new(rhs), operator: UnaryOperator::Minus }
            }
            Token::Plus => {
                let rhs = self.parse_expression(100)?;
                Expression::UnaryOperation { operand: Box::new(rhs), operator: UnaryOperator::Plus }
            }
            Token::Keyword(Keyword::Not) => {
                let rhs = self.parse_expression(100)?;
                Expression::UnaryOperation { operand: Box::new(rhs), operator: UnaryOperator::Not }
            }
            other => return Err(format!("Unexpected prefix token: {:?}", other)),
        };

        //infix/postfix loop
        loop {
            let prec = self.infix_precedence(self.peek());
            if prec <= min_prec {
                break;
            }
            let tok = self.next();
            left = match tok {
                Token::Plus => {
                    let rhs = self.parse_expression(25)?;
                    Expression::BinaryOperation { left_operand: Box::new(left), operator: BinaryOperator::Plus, right_operand: Box::new(rhs) }
                }
                Token::Minus => {
                    let rhs = self.parse_expression(25)?;
                    Expression::BinaryOperation { left_operand: Box::new(left), operator: BinaryOperator::Minus, right_operand: Box::new(rhs) }
                }
                Token::Star => {
                    let rhs = self.parse_expression(30)?;
                    Expression::BinaryOperation { left_operand: Box::new(left), operator: BinaryOperator::Multiply, right_operand: Box::new(rhs) }
                }
                Token::Divide => {
                    let rhs = self.parse_expression(30)?;
                    Expression::BinaryOperation { left_operand: Box::new(left), operator: BinaryOperator::Divide, right_operand: Box::new(rhs) }
                }
                Token::GreaterThan => {
                    let rhs = self.parse_expression(20)?;
                    Expression::BinaryOperation { left_operand: Box::new(left), operator: BinaryOperator::GreaterThan, right_operand: Box::new(rhs) }
                }
                Token::Keyword(Keyword::And) => {
                    let rhs = self.parse_expression(10)?;
                    Expression::BinaryOperation { left_operand: Box::new(left), operator: BinaryOperator::And, right_operand: Box::new(rhs) }
                }
                Token::Keyword(Keyword::Or) => {
                    let rhs = self.parse_expression(15)?;
                    Expression::BinaryOperation { left_operand: Box::new(left), operator: BinaryOperator::Or, right_operand: Box::new(rhs) }
                }
                Token::Keyword(Keyword::Asc) => {
                    Expression::UnaryOperation { operand: Box::new(left), operator: UnaryOperator::Asc }
                }
                Token::Keyword(Keyword::Desc) => {
                    Expression::UnaryOperation { operand: Box::new(left), operator: UnaryOperator::Desc }
                }
                Token::Equal => {
                    let rhs = self.parse_expression(20)?;
                    Expression::BinaryOperation { left_operand: Box::new(left), operator: BinaryOperator::Equal, right_operand: Box::new(rhs) }
                }
                Token::NotEqual => {
                    let rhs = self.parse_expression(20)?;
                    Expression::BinaryOperation { left_operand: Box::new(left), operator: BinaryOperator::NotEqual, right_operand: Box::new(rhs) }
                }
                Token::LessThan => {
                    let rhs = self.parse_expression(20)?;
                    Expression::BinaryOperation { left_operand: Box::new(left), operator: BinaryOperator::LessThan, right_operand: Box::new(rhs) }
                }
                Token::GreaterThanOrEqual => {
                    let rhs = self.parse_expression(20)?;
                    Expression::BinaryOperation { left_operand: Box::new(left), operator: BinaryOperator::GreaterThanOrEqual, right_operand: Box::new(rhs) }
                }
                Token::LessThanOrEqual => {
                    let rhs = self.parse_expression(20)?;
                    Expression::BinaryOperation { left_operand: Box::new(left), operator: BinaryOperator::LessThanOrEqual, right_operand: Box::new(rhs) }
                }
                _ => break,
            };
        }

        Ok(left)
    }

    //return precedence of infix or postfix tokens
    fn infix_precedence(&self, tok: &Token) -> u8 {
        match tok {
            Token::Plus | Token::Minus => 25,
            Token::Star | Token::Divide => 30,
            Token::GreaterThan | Token::LessThan | Token::Equal | Token::NotEqual
            | Token::GreaterThanOrEqual | Token::LessThanOrEqual => 20,
            Token::Keyword(Keyword::Or) => 15,
            Token::Keyword(Keyword::And) => 10,
            Token::Keyword(Keyword::Asc) | Token::Keyword(Keyword::Desc) => 5,
            _ => 0,
        }
    }
}