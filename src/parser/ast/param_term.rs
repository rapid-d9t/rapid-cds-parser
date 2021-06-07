use super::name_term::NameTerm;
use super::traits::ast_term::ASTTerm;
use crate::ir::argument_ir::ArgumentIR;
use crate::ir::ir_component::IRComponent;

pub struct ParamTerm {
    name: NameTerm,
    type_name: NameTerm,
}

impl ParamTerm {
    pub fn new(name: NameTerm, type_name: NameTerm) -> ParamTerm {
        ParamTerm { name, type_name }
    }

    pub fn get_name(&self) -> String {
        self.name.get_value()
    }

    pub fn get_type_name(&self) -> String {
        self.type_name.get_value()
    }
}

impl ASTTerm for ParamTerm {
    fn generate_ir(&self) -> Box<dyn IRComponent> {
        Box::new(ArgumentIR::new(
            self.get_name(),
            self.get_type_name(),
            false,
        ))
    }
}
