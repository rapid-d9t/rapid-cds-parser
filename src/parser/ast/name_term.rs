use super::traits::ast_term::ASTTerm;
use crate::ir::ir_component::IRComponent;

pub struct NameTerm {
    value: String,
}

impl NameTerm {
    pub fn new_boxed(value: String) -> Box<NameTerm> {
        Box::new(NameTerm::new(value))
    }

    pub fn new(value: String) -> NameTerm {
        NameTerm { value }
    }
}

impl ASTTerm for NameTerm {
    fn generate_ir(&self) -> Box<IRComponent> {
        Box::new(IRComponent::new_string(self.value.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::NameTerm;
    use crate::ir::ir_component::IRComponent;
    use crate::parser::ast::traits::ast_term::ASTTerm;

    #[test]
    fn it_generates_ir() {
        let name_term = NameTerm::new_boxed("mock-name".to_string());
        let name_ir = name_term.generate_ir();

        let correct_ir = IRComponent::new_string("mock-name".to_string());

        assert_eq!(name_ir, Box::new(correct_ir));
    }
}
