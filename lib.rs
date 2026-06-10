pub mod compiler {
    pub mod ast;
    pub mod lexer;
    pub mod parser;
    pub mod codegen;
    pub mod typecheck;
}

pub mod vm {
    pub mod value;
    pub mod bytecode;
    pub mod vm;
    pub mod syscalls;
}

pub mod zpm {
    pub mod zpm;
}
