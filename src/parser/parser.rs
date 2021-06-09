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

#[cfg(test)]
mod tests {
    use std::fs::remove_file;
    use std::fs::File;
    use std::io::prelude::*;

    use super::Parser;

    #[test]
    fn with_correct_input_it_translates() {
        let mut test_file = File::create("test_correct.cds").unwrap();
        test_file
            .write_all(
                b"
                    service TestService {
                        entity Test2 : Aspect1 {
                            field3 : Test3;
                        }
                    }
                ",
            )
            .unwrap();

        let result = Parser::new("test_correct.cds".to_string()).parse();

        remove_file("test_correct.cds").unwrap();

        assert!(result.is_ok());
    }

    #[test]
    fn with_unexisting_file_it_fails() {
        let result = Parser::new("test.cds".to_string()).parse();

        assert!(result.is_err());
    }

    #[test]
    fn with_syntactically_incorrect_it_fails() {
        let mut test_file = File::create("test_incorrect.cds").unwrap();
        test_file
            .write_all(
                b"
                    service TestService {
                        function ftest0() returns Test;
                        function ftest1(turns Test;
                        function ftest2(arg1: Test) returns array of Test;
                    }
                ",
            )
            .unwrap();

        let result = Parser::new("test_incorrect.cds".to_string()).parse();

        remove_file("test_incorrect.cds").unwrap();

        assert!(result.is_err());
    }
}
