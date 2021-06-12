use super::traits::ast_term::ASTTerm;
use super::traits::module_term_type::ModuleTermType;
use super::traits::module_usable_term::ModuleUsableTerm;
use super::traits::service_term_type::ServiceTermType;
use super::traits::service_usable_term::ServiceUsableTerm;
use crate::ir::ir_component::IRComponent;
use std::collections::HashMap;

pub struct ServiceTerm {
    name: Box<dyn ASTTerm>,
    definitions: Vec<Box<dyn ServiceUsableTerm>>,
}

impl ServiceTerm {
    pub fn new_boxed(
        name: Box<dyn ASTTerm>,
        definitions: Vec<Box<dyn ServiceUsableTerm>>,
    ) -> Box<ServiceTerm> {
        Box::new(ServiceTerm::new(name, definitions))
    }

    pub fn new(
        name: Box<dyn ASTTerm>,
        definitions: Vec<Box<dyn ServiceUsableTerm>>,
    ) -> ServiceTerm {
        ServiceTerm { name, definitions }
    }
}

impl ModuleUsableTerm for ServiceTerm {
    fn get_type(&self) -> ModuleTermType {
        ModuleTermType::Service
    }
}

impl ASTTerm for ServiceTerm {
    fn generate_ir(&self) -> Box<IRComponent> {
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

        let mut fields = HashMap::<String, Box<IRComponent>>::new();
        fields.insert("name".to_string(), self.name.generate_ir());
        fields.insert(
            "entities".to_string(),
            Box::new(IRComponent::new_array(entities)),
        );
        fields.insert(
            "actions".to_string(),
            Box::new(IRComponent::new_array(actions)),
        );
        fields.insert(
            "functions".to_string(),
            Box::new(IRComponent::new_array(functions)),
        );

        Box::new(IRComponent::new_object(fields))
    }
}

#[cfg(test)]
mod tests {
    use super::ServiceTerm;
    use crate::parser::ast::entity_term::EntityTerm;
    use crate::parser::ast::name_term::NameTerm;

    use crate::parser::ast::traits::module_term_type::ModuleTermType;
    use crate::parser::ast::traits::module_usable_term::ModuleUsableTerm;

    #[test]
    fn it_implements_module_usable_term_trait() {
        let service_name = NameTerm::new_boxed("Test".to_string());
        let entity_name = NameTerm::new_boxed("TestEntity".to_string());
        let entity = EntityTerm::new_boxed(entity_name, Vec::new(), Vec::new());
        let term = ServiceTerm::new(service_name, vec![entity]);
        let term: Box<dyn ModuleUsableTerm> = Box::new(term);

        assert_eq!(term.get_type(), ModuleTermType::Service);
    }
}
