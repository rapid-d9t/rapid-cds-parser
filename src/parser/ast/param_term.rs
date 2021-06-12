use super::traits::ast_term::ASTTerm;
use crate::ir::ir_component::IRComponent;
use std::collections::HashMap;

pub struct ParamTerm {
    name: Box<dyn ASTTerm>,
    type_name: Box<dyn ASTTerm>,
}

impl ParamTerm {
    pub fn new_boxed(name: Box<dyn ASTTerm>, type_name: Box<dyn ASTTerm>) -> Box<ParamTerm> {
        Box::new(ParamTerm::new(name, type_name))
    }

    pub fn new(name: Box<dyn ASTTerm>, type_name: Box<dyn ASTTerm>) -> ParamTerm {
        ParamTerm { name, type_name }
    }
}

impl ASTTerm for ParamTerm {
    fn generate_ir(&self) -> Box<IRComponent> {
        let mut fields = HashMap::<String, Box<IRComponent>>::new();
        fields.insert("name".to_string(), self.name.generate_ir());
        fields.insert("type".to_string(), self.type_name.generate_ir());
        fields.insert(
            "isArrayed".to_string(),
            Box::new(IRComponent::new_bool(false)),
        );

        Box::new(IRComponent::new_object_from_map(fields))
    }
}
