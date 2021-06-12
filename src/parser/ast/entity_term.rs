use super::traits::ast_term::ASTTerm;
use super::traits::module_term_type::ModuleTermType;
use super::traits::module_usable_term::ModuleUsableTerm;
use super::traits::service_term_type::ServiceTermType;
use super::traits::service_usable_term::ServiceUsableTerm;
use crate::ir::ir_component::IRComponent;
use std::collections::HashMap;

pub struct EntityTerm {
    name: Box<dyn ASTTerm>,
    applied_aspects: Vec<Box<dyn ASTTerm>>,
    fields: Vec<Box<dyn ASTTerm>>,
}

impl EntityTerm {
    pub fn new_boxed(
        name: Box<dyn ASTTerm>,
        applied_aspects: Vec<Box<dyn ASTTerm>>,
        fields: Vec<Box<dyn ASTTerm>>,
    ) -> Box<EntityTerm> {
        Box::new(EntityTerm::new(name, applied_aspects, fields))
    }

    pub fn new(
        name: Box<dyn ASTTerm>,
        applied_aspects: Vec<Box<dyn ASTTerm>>,
        fields: Vec<Box<dyn ASTTerm>>,
    ) -> EntityTerm {
        EntityTerm {
            name,
            applied_aspects,
            fields,
        }
    }
}

impl ModuleUsableTerm for EntityTerm {
    fn get_type(&self) -> ModuleTermType {
        ModuleTermType::Entity
    }
}

impl ServiceUsableTerm for EntityTerm {
    fn get_type(&self) -> ServiceTermType {
        ServiceTermType::Entity
    }
}

impl ASTTerm for EntityTerm {
    fn generate_ir(&self) -> Box<IRComponent> {
        let aspects = self
            .applied_aspects
            .iter()
            .map(|aspect| aspect.generate_ir())
            .collect();

        let fields_arr = self
            .fields
            .iter()
            .map(|field| field.generate_ir())
            .collect();

        let mut fields = HashMap::<String, Box<IRComponent>>::new();
        fields.insert("name".to_string(), self.name.generate_ir());
        fields.insert(
            "aspects".to_string(),
            Box::new(IRComponent::new_array(aspects)),
        );
        fields.insert(
            "fields".to_string(),
            Box::new(IRComponent::new_array(fields_arr)),
        );
        fields.insert(
            "isProjection".to_string(),
            Box::new(IRComponent::new_bool(false)),
        );

        Box::new(IRComponent::new_object(fields))
    }
}
