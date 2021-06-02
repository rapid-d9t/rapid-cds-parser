use neon::prelude::*;
use std::io::prelude::*;

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub cds);

use std::fs::File;
use std::path::Path;

pub mod ast;
use ast::traits::ast_term::ASTTerm;

fn transpile(mut cx: FunctionContext) -> JsResult<JsString> {
    let path = cx.argument::<JsString>(0)?.value(&mut cx);

    let path = Path::new(&path);

    let mut file = File::open(path).unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let module = cds::ModuleParser::new().parse(&content).unwrap();

    Ok(cx.string(module.convert_to_json()))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("transpile", transpile)?;
    Ok(())
}
