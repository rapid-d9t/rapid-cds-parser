use std::io::prelude::*;

use std::fs::File;
use std::path::Path;

use super::ast::module_term::ModuleTerm;
use super::parse_error::ParseError;
use super::parse_error::ParseErrorType;

pub struct Parser {
    path: String,
}

impl Parser {
    pub fn new(path: String) -> Parser {
        Parser { path }
    }

    pub fn parse(&self) -> Result<ModuleTerm, ParseError> {
        let path = Path::new(&self.path);

        let mut file = File::open(path)?;

        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let module = match super::cds::ModuleParser::new().parse(&content) {
            Ok(module_ast) => module_ast,
            Err(lalrpop_auto_generated_error) => {
                return Err(ParseError::new(
                    format!("{}", lalrpop_auto_generated_error),
                    ParseErrorType::SyntaxError,
                ))
            }
        };

        Ok(module)
    }
}
