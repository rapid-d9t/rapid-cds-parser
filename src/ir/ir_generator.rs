use crate::parser::parser::Parser;

pub struct IRGenerator {
    parser: Parser,
}

impl IRGenerator {
    pub fn new(src_path: String) -> IRGenerator {
        IRGenerator {
            parser: Parser::new(src_path),
        }
    }

    pub fn generate(&self) -> String {
        self.parser.parse()
    }
}
