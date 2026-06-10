use zlang_core::compiler::codegen::Codegen;
use zlang_core::compiler::lexer::Lexer;
use zlang_core::compiler::parser::Parser;
use zlang_core::vm::vm::VM;

fn main() {
    println!("ZLang CLI v0.1.0 - Full Test");

    let inputs = vec![
        "let a = 10 + 20",
        "let b = \"ZDOS \" + \"ZLang\"",
        "let c = 100 - 50 + 25",
    ];

    for input in inputs {
        println!("\nTesting: {}", input);
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let stmts = parser.parse_program();

        let mut codegen = Codegen::new();
        codegen.compile(&stmts);

        let mut vm = VM::new(codegen.code, codegen.constants);
        vm.run();

        if let Some(result) = vm.stack.last() {
            println!("Result: {:?}", result);
        } else {
            println!("No result (stack empty)");
        }
    }
}
