use super::field_term::FieldTerm;
use super::name_term::NameTerm;
use super::traits::ast_term::ASTTerm;
use super::traits::module_term_type::ModuleTermType;
use super::traits::module_usable_term::ModuleUsableTerm;
use super::traits::service_term_type::ServiceTermType;
use super::traits::service_usable_term::ServiceUsableTerm;
use crate::ir::entity_ir::EntityIR;
use crate::ir::ir_component::IRComponent;

pub struct EntityTerm {
    name: NameTerm,
    applied_aspects: Vec<NameTerm>,
    fields: Vec<FieldTerm>,
}

impl EntityTerm {
    pub fn new(
        name: NameTerm,
        applied_aspects: Vec<NameTerm>,
        fields: Vec<FieldTerm>,
    ) -> EntityTerm {
        EntityTerm {
            name,
            applied_aspects,
            fields,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.get_value()
    }
}

impl ModuleUsableTerm for EntityTerm {
    fn get_type(&self) -> ModuleTermType {
        ModuleTermType::Entity
    }
}

impl ServiceUsableTerm for EntityTerm {
    fn get_name(&self) -> String {
        self.name.get_value()
    }

    fn get_type(&self) -> ServiceTermType {
        ServiceTermType::Entity
    }
}

impl ASTTerm for EntityTerm {
    fn generate_ir(&self) -> Box<dyn IRComponent> {
        let aspects = self
            .applied_aspects
            .iter()
            .map(|aspect| aspect.get_value())
            .collect();

        let fields = self
            .fields
            .iter()
            .map(|field| field.generate_ir())
            .collect();

        Box::new(EntityIR::new_concrete(self.get_name(), aspects, fields))
    }
}
