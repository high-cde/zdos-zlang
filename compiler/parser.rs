use crate::compiler::ast::{BinaryOp, Expr, Literal, Stmt};
use crate::compiler::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut p = Parser {
            lexer,
            current_token: Token::EOF,
        };
        p.advance();
        p
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.current_token != Token::EOF {
            stmts.push(self.parse_statement());
        }
        stmts
    }

    fn parse_statement(&mut self) -> Stmt {
        match self.current_token {
            Token::Let => self.parse_var_decl(),
            _ => Stmt::Expr(self.parse_expression()),
        }
    }

    fn parse_var_decl(&mut self) -> Stmt {
        self.advance(); // skip let
        if let Token::Identifier(name) = self.current_token.clone() {
            self.advance();
            let mut type_name = None;
            if self.current_token == Token::Colon {
                self.advance();
                if let Token::Identifier(t) = self.current_token.clone() {
                    type_name = Some(t);
                    self.advance();
                }
            }
            if self.current_token == Token::Eq {
                self.advance();
                let expr = self.parse_expression();
                return Stmt::VarDecl(name, type_name, expr);
            }
        }
        panic!("Expected identifier in var decl");
    }

    fn parse_expression(&mut self) -> Expr {
        self.parse_additive()
    }

    fn parse_additive(&mut self) -> Expr {
        let mut left = self.parse_primary();
        while matches!(self.current_token, Token::Plus | Token::Minus) {
            let op = if self.current_token == Token::Plus {
                BinaryOp::Add
            } else {
                BinaryOp::Sub
            };
            self.advance();
            let right = self.parse_primary();
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        left
    }

    fn parse_primary(&mut self) -> Expr {
        match self.current_token.clone() {
            Token::Int(i) => {
                self.advance();
                Expr::Literal(Literal::Int(i))
            }
            Token::Str(s) => {
                self.advance();
                Expr::Literal(Literal::Str(s))
            }
            Token::Identifier(id) => {
                self.advance();
                Expr::Identifier(id)
            }
            _ => panic!("Unexpected token in primary: {:?}", self.current_token),
        }
    }
}
