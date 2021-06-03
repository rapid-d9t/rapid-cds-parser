mod ir;
pub mod parser;

use ir::ir_generator::IRGenerator;
use neon::prelude::*;

fn generate_ir(mut cx: FunctionContext) -> JsResult<JsString> {
    let path = cx.argument::<JsString>(0)?.value(&mut cx);

    let generator = IRGenerator::new(path);

    Ok(cx.string(generator.generate()))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("generate_ir", generate_ir)?;
    Ok(())
}
