use super::name_term::NameTerm;
use super::traits::ast_term::ASTTerm;
use crate::ir::ir_component::IRComponent;
use std::collections::HashMap;

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
    fn generate_ir(&self) -> Box<IRComponent> {
        let mut fields = HashMap::<String, Box<IRComponent>>::new();
        fields.insert("name".to_string(), self.name.generate_ir());
        fields.insert("type".to_string(), self.type_name.generate_ir());
        fields.insert(
            "hasDefault".to_string(),
            Box::new(IRComponent::new_bool(false)),
        );

        Box::new(IRComponent::new_object(fields))
    }
}
