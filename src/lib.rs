mod ir;
pub mod parser;

use ir::ir_generator::IRGenerator;
use neon::prelude::*;

fn generate_ir(mut cx: FunctionContext) -> JsResult<JsObject> {
    let path = cx.argument::<JsString>(0)?.value(&mut cx);

    let generator = IRGenerator::new(path);

    cx.compute_scoped(|mut cx| generator.generate(&mut cx))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("generate_ir", generate_ir)?;
    Ok(())
}
