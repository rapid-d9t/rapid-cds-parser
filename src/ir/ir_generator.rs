use crate::parser::ast::traits::ast_term::ASTTerm;
use crate::parser::parser::Parser;
use neon::prelude::*;

pub struct IRGenerator {
    parser: Parser,
}

impl IRGenerator {
    pub fn new(src_path: String) -> IRGenerator {
        IRGenerator {
            parser: Parser::new(src_path),
        }
    }

    pub fn generate<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
    ) -> neon::handle::Handle<'internal, JsObject> {
        let root_module = self.parser.parse();

        let ir_representation = root_module.generate_ir();

        cx.compute_scoped(|mut cx| Ok(ir_representation.to_js_object(&mut cx).unwrap()))
            .unwrap()
    }
}
