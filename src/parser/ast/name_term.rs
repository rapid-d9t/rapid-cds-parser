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
