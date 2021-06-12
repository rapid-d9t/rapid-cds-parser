use super::traits::ast_term::ASTTerm;
use super::traits::module_term_type::ModuleTermType;
use super::traits::module_usable_term::ModuleUsableTerm;
use crate::ir::ir_component::IRComponent;
use std::collections::HashMap;

pub struct ModuleTerm {
    definitions: Vec<Box<dyn ModuleUsableTerm>>,
}

impl ModuleTerm {
    pub fn new_boxed(definitions: Vec<Box<dyn ModuleUsableTerm>>) -> Box<ModuleTerm> {
        Box::new(ModuleTerm::new(definitions))
    }

    pub fn new(definitions: Vec<Box<dyn ModuleUsableTerm>>) -> ModuleTerm {
        ModuleTerm { definitions }
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

        Box::new(IRComponent::new_object_from_map(fields))
    }
}
