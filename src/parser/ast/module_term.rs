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

#[cfg(test)]
mod tests {
    use crate::parser::ast::traits::ast_term::ASTTerm;

    use crate::ir::ir_component::IRComponent;

    use super::ModuleTerm;

    #[test]
    fn it_generates_ir() {
        let service_term = ModuleTerm::new_boxed(vec![]);
        let service_ir = service_term.generate_ir();

        let correct_ir_mock_fields = vec![
            ("entities", Box::new(IRComponent::new_array(vec![]))),
            ("services", Box::new(IRComponent::new_array(vec![]))),
            ("types", Box::new(IRComponent::new_array(vec![]))),
        ];
        let correct_ir = IRComponent::new_object_from_vec(correct_ir_mock_fields);

        assert_eq!(service_ir, Box::new(correct_ir));
    }
}
