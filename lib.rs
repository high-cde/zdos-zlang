#![allow(clippy::module_inception)]

pub mod compiler {
    pub mod ast;
    pub mod codegen;
    pub mod lexer;
    pub mod parser;
    pub mod typecheck;
}

pub mod vm {
    pub mod bytecode;
    pub mod syscalls;
    pub mod value;
    pub mod vm;
}

pub mod zpm {
    pub mod zpm;
}
