use crate::compiler::ast::{BinaryOp, Expr, Literal, Stmt};
use crate::vm::bytecode::OpCode;
use crate::vm::value::Value;

pub struct Codegen {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
}

impl Default for Codegen {
    fn default() -> Self {
        Self::new()
    }
}

impl Codegen {
    pub fn new() -> Self {
        Codegen {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn compile(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.compile_stmt(stmt);
        }
        self.emit_byte(OpCode::Ret as u8);
    }

    fn compile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::VarDecl(_name, _type_name, expr) => {
                self.compile_expr(expr);
            }
            Stmt::Expr(expr) => {
                self.compile_expr(expr);
                self.emit_byte(OpCode::Pop as u8);
            }
            _ => {}
        }
    }

    fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Literal(lit) => match lit {
                Literal::Int(i) => {
                    let idx = self.add_constant(Value::Int(*i));
                    self.emit_byte(OpCode::PushConst as u8);
                    self.emit_byte(idx as u8);
                }
                Literal::Str(s) => {
                    let idx = self.add_constant(Value::Str(s.clone()));
                    self.emit_byte(OpCode::PushConst as u8);
                    self.emit_byte(idx as u8);
                }
                _ => {}
            },
            Expr::Binary(left, op, right) => {
                self.compile_expr(left);
                self.compile_expr(right);
                match op {
                    BinaryOp::Add => self.emit_byte(OpCode::Add as u8),
                    BinaryOp::Sub => self.emit_byte(OpCode::Sub as u8),
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn add_constant(&mut self, val: Value) -> usize {
        self.constants.push(val);
        self.constants.len() - 1
    }

    fn emit_byte(&mut self, byte: u8) {
        self.code.push(byte);
    }
}
