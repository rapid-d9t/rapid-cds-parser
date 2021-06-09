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
    definitions: Vec<Box<dyn ServiceUsableTerm>>,
}

impl ServiceTerm {
    pub fn new(name: NameTerm, definitions: Vec<Box<dyn ServiceUsableTerm>>) -> ServiceTerm {
        ServiceTerm { name, definitions }
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

#[cfg(test)]
mod tests {
    use super::ServiceTerm;
    use crate::parser::ast::entity_term::EntityTerm;
    use crate::parser::ast::name_term::NameTerm;

    use crate::parser::ast::traits::service_usable_term::ServiceUsableTerm;

    use crate::parser::ast::traits::module_term_type::ModuleTermType;
    use crate::parser::ast::traits::module_usable_term::ModuleUsableTerm;

    #[test]
    fn it_inits() {
        let service_name = NameTerm::new("Test".to_string());
        let entity_name = NameTerm::new("TestEntity".to_string());
        let entity = EntityTerm::new(entity_name, Vec::new(), Vec::new());
        let term = ServiceTerm::new(service_name, vec![Box::new(entity)]);

        assert_eq!(term.get_name(), "Test");
    }

    #[test]
    fn it_implements_service_usable_term_trait() {
        let service_name = NameTerm::new("Test".to_string());
        let entity_name = NameTerm::new("TestEntity".to_string());
        let entity = EntityTerm::new(entity_name, Vec::new(), Vec::new());
        let term = ServiceTerm::new(service_name, vec![Box::new(entity)]);
        let term: Box<dyn ModuleUsableTerm> = Box::new(term);

        assert_eq!(term.get_type(), ModuleTermType::Service);
    }
}
