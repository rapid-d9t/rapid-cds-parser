use super::traits::ast_term::ASTTerm;
use crate::ir::ir_component::IRComponent;

pub struct NameTerm {
    value: String,
}

impl NameTerm {
    pub fn new(value: String) -> NameTerm {
        NameTerm { value }
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }
}

impl ASTTerm for NameTerm {
    fn generate_ir(&self) -> Box<IRComponent> {
        Box::new(IRComponent::new_string(self.value.clone()))
    }
}
