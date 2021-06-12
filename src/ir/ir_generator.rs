use super::js_context::JsContext;
use crate::parser::ast::module_term::ModuleTerm;
use crate::parser::ast::traits::ast_term::ASTTerm;
use crate::parser::parser::Parser;
use neon::prelude::*;

#[cfg(not(tarpaulin_include))]
pub struct IRGenerator {
    parser: Parser,
}

#[cfg(not(tarpaulin_include))]
impl IRGenerator {
    pub fn new(src_path: String) -> IRGenerator {
        IRGenerator {
            parser: Parser::new(src_path),
        }
    }

    pub fn generate<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
    ) -> JsResult<'internal, JsValue> {
        let root_module = self.parse(cx)?;

        self.generate_ir_object_tree(root_module, cx)
    }

    fn parse<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
    ) -> Result<Box<ModuleTerm>, neon::result::Throw> {
        match self.parser.parse() {
            Ok(module) => Ok(module),
            Err(parse_error) => return cx.throw_error(format!("{}", parse_error)),
        }
    }

    fn generate_ir_object_tree<'internal, 'outer>(
        &self,
        root_module: Box<ModuleTerm>,
        cx: &mut JsContext<'internal, 'outer>,
    ) -> JsResult<'internal, JsValue> {
        let ir_representation = root_module.generate_ir();

        match ir_representation.to_js_value(cx) {
            Ok(object) => Ok(object),
            Err(ir_generation_error) => return cx.throw_error(format!("{}", ir_generation_error)),
        }
    }
}
