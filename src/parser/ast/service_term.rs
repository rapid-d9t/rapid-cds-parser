use super::name_term::NameTerm;
use super::traits::ast_term::ASTTerm;
use super::traits::module_term_type::ModuleTermType;
use super::traits::module_usable_term::ModuleUsableTerm;
use super::traits::service_term_type::ServiceTermType;
use super::traits::service_usable_term::ServiceUsableTerm;
use crate::ir::ir_component::IRComponent;
use crate::ir::service_ir::ServiceIR;

pub struct ServiceTerm {
    name: NameTerm,
    applied_aspects: Vec<NameTerm>,
    definitions: Vec<Box<dyn ServiceUsableTerm>>,
}

impl ServiceTerm {
    pub fn new(
        name: NameTerm,
        applied_aspects: Vec<NameTerm>,
        definitions: Vec<Box<dyn ServiceUsableTerm>>,
    ) -> ServiceTerm {
        ServiceTerm {
            name,
            applied_aspects,
            definitions,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.get_value()
    }
}

impl ModuleUsableTerm for ServiceTerm {
    fn get_type(&self) -> ModuleTermType {
        ModuleTermType::Service
    }
}

impl ASTTerm for ServiceTerm {
    fn generate_ir(&self) -> Box<dyn IRComponent> {
        let entities = self
            .definitions
            .iter()
            .filter(|definition| definition.get_type() == ServiceTermType::Entity)
            .map(|entity| entity.generate_ir())
            .collect();

        let actions = self
            .definitions
            .iter()
            .filter(|definition| definition.get_type() == ServiceTermType::Action)
            .map(|entity| entity.generate_ir())
            .collect();

        let functions = self
            .definitions
            .iter()
            .filter(|definition| definition.get_type() == ServiceTermType::Function)
            .map(|function| function.generate_ir())
            .collect();

        Box::new(ServiceIR::new(
            self.get_name(),
            entities,
            actions,
            functions,
        ))
    }
}
