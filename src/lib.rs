mod codegen;
mod effects;
mod lexing;
mod parsing;
mod types;

use crate::codegen::wasm::WasmCompiler;
use crate::lexing::token::Token;
use crate::parsing::ast::Program;
use crate::types::TypeEnv;
use logos::Logos;

pub fn compile(source: &str) -> Result<Vec<u8>, String> {
    let tokens = Token::lexer(source).collect::<Vec<_>>();
    let ast = parse_tokens(tokens)?;
    let type_env = type_check(&ast)?;
    let wasm = WasmCompiler::new().compile(&ast);
    Ok(wasm)
}

fn parse_tokens(tokens: Vec<Token>) -> Result<Program, String> {
    // Implement parser
    unimplemented!("Parser not yet implemented")
}

fn type_check(ast: &Program) -> Result<TypeEnv, String> {
    // Implement type checker
    unimplemented!("Type checker not yet implemented")
}
