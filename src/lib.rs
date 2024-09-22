mod codegen;
mod effects;
mod lexing;
mod parsing;
mod types;

use codegen::wasm::WasmCompiler;
use parsing::parser::parse;

pub fn compile(source: &str) -> Result<Vec<u8>, String> {
    let ast = parse(source)?;
    let wasm = WasmCompiler::new().compile(&ast);
    Ok(wasm)
}

