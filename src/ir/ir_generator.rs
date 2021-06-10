use super::js_context::JsContext;
use crate::parser::ast::module_term::ModuleTerm;
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
        cx: &mut JsContext<'internal, 'outer>,
    ) -> JsResult<'internal, JsObject> {
        let root_module = self.parse(cx)?;

        self.generate_ir_object_tree(root_module, cx)
    }

    fn parse<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
    ) -> Result<ModuleTerm, neon::result::Throw> {
        match self.parser.parse() {
            Ok(module) => Ok(module),
            Err(parse_error) => return cx.throw_error(format!("{}", parse_error)),
        }
    }

    fn generate_ir_object_tree<'internal, 'outer>(
        &self,
        root_module: ModuleTerm,
        cx: &mut JsContext<'internal, 'outer>,
    ) -> JsResult<'internal, JsObject> {
        let ir_representation = root_module.generate_ir();

        match ir_representation.to_js_object(cx) {
            Ok(object) => Ok(object),
            Err(ir_generation_error) => return cx.throw_error(format!("{}", ir_generation_error)),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use neon::prelude::*;

//     use mockall::mock;

//     use std::fs::remove_file;
//     use std::fs::File;
//     use std::io::prelude::*;

//     use super::IRGenerator;

//     mock!{
//         struct JsContext<'internal, 'external> {

//         }
//     }

//     #[test]
//     fn with_correct_input_it_generates() {
//         let mut test_file = File::create("gen_test_correct.cds").unwrap();
//         test_file
//             .write_all(
//                 b"
//                     service TestService {
//                         entity Test2 : Aspect1 {
//                             field3 : Test3;
//                         }
//                     }
//                 ",
//             )
//             .unwrap();

//         let result = IRGenerator::new("gen_test_correct.cds".to_string()).generate();

//         remove_file("gen_test_correct.cds").unwrap();

//         assert!(result.is_ok());
//     }
// }
