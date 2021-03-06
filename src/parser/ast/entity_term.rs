use super::traits::ast_term::ASTTerm;
use super::traits::module_term_type::ModuleTermType;
use super::traits::module_usable_term::ModuleUsableTerm;
use super::traits::service_term_type::ServiceTermType;
use super::traits::service_usable_term::ServiceUsableTerm;
use crate::ir::ir_component::IRComponent;

pub struct EntityTerm {
    name: Box<dyn ASTTerm>,
    applied_aspects: Vec<Box<dyn ASTTerm>>,
    fields: Vec<Box<dyn ASTTerm>>,
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
        let fields = vec![
            ("name", self.name.generate_ir()),
            ("aspects", self.generate_aspects_ir()),
            ("fields", self.generate_fields_ir()),
            ("isProjection", Box::new(IRComponent::Bool(false))),
        ];

        Box::new(IRComponent::new_object_from_vec(fields))
    }
}

impl EntityTerm {
    fn generate_aspects_ir(&self) -> Box<IRComponent> {
        let aspects = self
            .applied_aspects
            .iter()
            .map(|aspect| aspect.generate_ir())
            .collect();

        Box::new(IRComponent::new_array(aspects))
    }

    fn generate_fields_ir(&self) -> Box<IRComponent> {
        let fields = self
            .fields
            .iter()
            .map(|field| field.generate_ir())
            .collect();

        Box::new(IRComponent::new_array(fields))
    }

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

#[cfg(test)]
mod tests {
    use super::EntityTerm;
    use crate::ir::ir_component::IRComponent;
    use crate::parser::ast::name_term::NameTerm;
    use crate::parser::ast::traits::ast_term::ASTTerm;
    use crate::parser::ast::traits::service_term_type::ServiceTermType;
    use crate::parser::ast::traits::service_usable_term::ServiceUsableTerm;

    #[test]
    fn it_implements_service_usable_term_trait() {
        let entity_term =
            EntityTerm::new_boxed(NameTerm::new_boxed("mock".to_string()), vec![], vec![]);

        assert_eq!(entity_term.get_type(), ServiceTermType::Entity);
    }

    #[test]
    fn with_empty_aspects_it_generates_ir() {
        let entity_term =
            EntityTerm::new_boxed(NameTerm::new_boxed("mock".to_string()), vec![], vec![]);
        let entity_ir = entity_term.generate_ir();

        let correct_ir_mock_fields = vec![
            (
                "name",
                Box::new(IRComponent::new_string("mock".to_string())),
            ),
            ("fields", Box::new(IRComponent::new_array(vec![]))),
            ("aspects", Box::new(IRComponent::new_array(vec![]))),
            ("isProjection", Box::new(IRComponent::new_bool(false))),
        ];
        let correct_ir = IRComponent::new_object_from_vec(correct_ir_mock_fields);

        assert_eq!(entity_ir, Box::new(correct_ir));
    }

    #[test]
    fn with_some_aspects_it_generates_ir() {
        let entity_term = EntityTerm::new_boxed(
            NameTerm::new_boxed("mock".to_string()),
            vec![NameTerm::new_boxed("mock".to_string())],
            vec![],
        );
        let entity_ir = entity_term.generate_ir();

        let correct_ir_mock_fields = vec![
            (
                "name",
                Box::new(IRComponent::new_string("mock".to_string())),
            ),
            ("fields", Box::new(IRComponent::new_array(vec![]))),
            (
                "aspects",
                Box::new(IRComponent::new_array(vec![Box::new(
                    IRComponent::new_string("mock".to_string()),
                )])),
            ),
            ("isProjection", Box::new(IRComponent::new_bool(false))),
        ];
        let correct_ir = IRComponent::new_object_from_vec(correct_ir_mock_fields);

        assert_eq!(entity_ir, Box::new(correct_ir));
    }
}
