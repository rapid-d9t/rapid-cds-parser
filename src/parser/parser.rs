use std::io::prelude::*;

use std::fs::File;
use std::path::Path;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub cds);

use super::ast::traits::ast_term::ASTTerm;

pub struct Parser {
    path: String,
}

impl Parser {
    pub fn new(path: String) -> Parser {
        Parser { path }
    }

    pub fn parse(&self) -> String {
        let path = Path::new(&self.path);

        let mut file = File::open(path).unwrap();

        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        let module = cds::ModuleParser::new().parse(&content).unwrap();

        module.convert_to_json()
    }
}
