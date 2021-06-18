use super::traits::ast_term::ASTTerm;
use crate::ir::ir_component::IRComponent;

pub struct ReturnsTerm {
    type_name: Box<dyn ASTTerm>,
    is_arrayed: bool,
}

impl ASTTerm for ReturnsTerm {
    fn generate_ir(&self) -> Box<IRComponent> {
        let fields = vec![
            ("type", self.type_name.generate_ir()),
            (
                "isArrayed",
                Box::new(IRComponent::new_bool(self.is_arrayed)),
            ),
        ];

        Box::new(IRComponent::new_object_from_vec(fields))
    }
}

impl ReturnsTerm {
    pub fn new_boxed(type_name: Box<dyn ASTTerm>) -> Box<ReturnsTerm> {
        Box::new(ReturnsTerm::new(type_name))
    }

    pub fn new(type_name: Box<dyn ASTTerm>) -> ReturnsTerm {
        ReturnsTerm {
            type_name,
            is_arrayed: false,
        }
    }

    pub fn new_arrayed_boxed(type_name: Box<dyn ASTTerm>) -> Box<ReturnsTerm> {
        Box::new(ReturnsTerm::new_arrayed(type_name))
    }

    pub fn new_arrayed(type_name: Box<dyn ASTTerm>) -> ReturnsTerm {
        ReturnsTerm {
            type_name,
            is_arrayed: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ReturnsTerm;
    use crate::ir::ir_component::IRComponent;
    use crate::parser::ast::name_term::NameTerm;
    use crate::parser::ast::traits::ast_term::ASTTerm;

    #[test]
    fn it_generates_ir() {
        let returns_term = ReturnsTerm::new_boxed(NameTerm::new_boxed("mock-type".to_string()));
        let returns_ir = returns_term.generate_ir();

        let correct_ir_mock_returns = vec![
            (
                "type",
                Box::new(IRComponent::new_string("mock-type".to_string())),
            ),
            ("isArrayed", Box::new(IRComponent::new_bool(false))),
        ];
        let correct_ir = IRComponent::new_object_from_vec(correct_ir_mock_returns);

        assert_eq!(returns_ir, Box::new(correct_ir));
    }

    #[test]
    fn with_arrayed_it_generates_ir() {
        let returns_term =
            ReturnsTerm::new_arrayed_boxed(NameTerm::new_boxed("mock-type".to_string()));
        let returns_ir = returns_term.generate_ir();

        let correct_ir_mock_returns = vec![
            (
                "type",
                Box::new(IRComponent::new_string("mock-type".to_string())),
            ),
            ("isArrayed", Box::new(IRComponent::new_bool(true))),
        ];
        let correct_ir = IRComponent::new_object_from_vec(correct_ir_mock_returns);

        assert_eq!(returns_ir, Box::new(correct_ir));
    }
}
