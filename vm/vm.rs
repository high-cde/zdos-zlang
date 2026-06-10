use crate::vm::bytecode::OpCode;
use crate::vm::value::Value;

pub struct VM {
    pub stack: Vec<Value>,
    pub ip: usize,
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
}

impl VM {
    pub fn new(code: Vec<u8>, constants: Vec<Value>) -> Self {
        VM {
            stack: Vec::new(),
            ip: 0,
            code,
            constants,
        }
    }

    pub fn run(&mut self) {
        while self.ip < self.code.len() {
            let opcode = OpCode::from(self.code[self.ip]);
            self.ip += 1;
            match opcode {
                OpCode::PushConst => {
                    let idx = self.code[self.ip] as usize;
                    self.ip += 1;
                    self.stack.push(self.constants[idx].clone());
                }
                OpCode::Pop => {
                    self.stack.pop();
                }
                OpCode::Add => {
                    let b = self.stack.pop().expect("Stack underflow");
                    let a = self.stack.pop().expect("Stack underflow");
                    match (a, b) {
                        (Value::Int(i1), Value::Int(i2)) => self.stack.push(Value::Int(i1 + i2)),
                        (Value::Float(f1), Value::Float(f2)) => {
                            self.stack.push(Value::Float(f1 + f2))
                        }
                        (Value::Str(s1), Value::Str(s2)) => {
                            self.stack.push(Value::Str(format!("{}{}", s1, s2)))
                        }
                        _ => panic!("Type error in ADD"),
                    }
                }
                OpCode::Sub => {
                    let b = self.stack.pop().expect("Stack underflow");
                    let a = self.stack.pop().expect("Stack underflow");
                    match (a, b) {
                        (Value::Int(i1), Value::Int(i2)) => self.stack.push(Value::Int(i1 - i2)),
                        (Value::Float(f1), Value::Float(f2)) => {
                            self.stack.push(Value::Float(f1 - f2))
                        }
                        _ => panic!("Type error in SUB"),
                    }
                }
                OpCode::Ret => return,
                _ => todo!("Opcode {:?} not implemented", opcode),
            }
        }
    }
}
