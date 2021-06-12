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

        Box::new(IRComponent::new_object_from_map(fields))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::ast::traits::ast_term::ASTTerm;

    use crate::ir::ir_component::IRComponent;

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
        let term: Box<dyn ModuleUsableTerm> = ServiceTerm::new_boxed(service_name, vec![entity]);

        assert_eq!(term.get_type(), ModuleTermType::Service);
    }

    #[test]
    fn it_generates_ir() {
        let service_name = NameTerm::new_boxed("Test".to_string());
        let entity_name = NameTerm::new_boxed("TestEntity".to_string());
        let entity = EntityTerm::new_boxed(entity_name, Vec::new(), Vec::new());
        let service_term = ServiceTerm::new_boxed(service_name, vec![entity]);
        let service_ir = service_term.generate_ir();

        let entity_fields = vec![
            (
                "name",
                Box::new(IRComponent::new_string("TestEntity".to_string())),
            ),
            ("fields", Box::new(IRComponent::new_array(vec![]))),
            ("aspects", Box::new(IRComponent::new_array(vec![]))),
            ("isProjection", Box::new(IRComponent::new_bool(false))),
        ];
        let correct_ir_mock_fields = vec![
            (
                "name",
                Box::new(IRComponent::new_string("Test".to_string())),
            ),
            (
                "entities",
                Box::new(IRComponent::new_array(vec![Box::new(
                    IRComponent::new_object_from_vec(entity_fields),
                )])),
            ),
            ("actions", Box::new(IRComponent::new_array(vec![]))),
            ("functions", Box::new(IRComponent::new_array(vec![]))),
        ];
        let correct_ir = IRComponent::new_object_from_vec(correct_ir_mock_fields);

        assert_eq!(service_ir, Box::new(correct_ir));
    }
}
