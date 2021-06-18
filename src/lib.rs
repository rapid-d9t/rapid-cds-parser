mod ir;
pub mod parser;

use ir::ir_generator::IRGenerator;
use ir::js_context::JsContext;
use neon::prelude::*;

#[cfg(not(tarpaulin_include))]
fn generate_ir(mut cx: FunctionContext) -> JsResult<JsValue> {
    let path = cx.argument::<JsString>(0)?.value(&mut cx);

    let generator = IRGenerator::new(path);

    cx.compute_scoped(|cx| {
        let mut context = JsContext::new(cx);
        generator.generate(&mut context)
    })
}

#[neon::main]
#[cfg(not(tarpaulin_include))]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("generateIR", generate_ir)?;
    Ok(())
}
