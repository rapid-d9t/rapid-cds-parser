use super::traits::ast_term::ASTTerm;
use super::traits::module_term_type::ModuleTermType;
use super::traits::module_usable_term::ModuleUsableTerm;
use crate::ir::ir_component::IRComponent;
use std::collections::HashMap;

pub struct ModuleTerm {
    name: String,
    definitions: Vec<Box<dyn ModuleUsableTerm>>,
}

impl ModuleTerm {
    pub fn new(definitions: Vec<Box<dyn ModuleUsableTerm>>) -> ModuleTerm {
        ModuleTerm {
            name: "".to_string(),
            definitions,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, new_name: &String) {
        self.name = new_name.clone();
    }
}

impl ASTTerm for ModuleTerm {
    fn generate_ir(&self) -> Box<IRComponent> {
        let entities = self
            .definitions
            .iter()
            .filter(|definition| definition.get_type() == ModuleTermType::Entity)
            .map(|entity| entity.generate_ir())
            .collect();

        let types = self
            .definitions
            .iter()
            .filter(|definition| definition.get_type() == ModuleTermType::Type)
            .map(|cds_type| cds_type.generate_ir())
            .collect();

        let services = self
            .definitions
            .iter()
            .filter(|definition| definition.get_type() == ModuleTermType::Service)
            .map(|service| service.generate_ir())
            .collect();

        let mut fields = HashMap::<String, Box<IRComponent>>::new();
        fields.insert(
            "entities".to_string(),
            Box::new(IRComponent::new_array(entities)),
        );
        fields.insert("types".to_string(), Box::new(IRComponent::new_array(types)));
        fields.insert(
            "services".to_string(),
            Box::new(IRComponent::new_array(services)),
        );

        Box::new(IRComponent::new_object(fields))
    }
}
