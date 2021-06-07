use super::name_term::NameTerm;
use super::traits::ast_term::ASTTerm;
use crate::ir::field_ir::FieldIR;
use crate::ir::ir_component::IRComponent;

pub struct FieldTerm {
    name: NameTerm,
    type_name: NameTerm,
}

impl FieldTerm {
    pub fn new(name: NameTerm, type_name: NameTerm) -> FieldTerm {
        FieldTerm { name, type_name }
    }

    pub fn get_name(&self) -> String {
        self.name.get_value()
    }

    pub fn get_type_name(&self) -> String {
        self.type_name.get_value()
    }
}

impl ASTTerm for FieldTerm {
    fn generate_ir(&self) -> Box<dyn IRComponent> {
        Box::new(FieldIR::new(self.get_name(), self.get_type_name(), None))
    }
}
