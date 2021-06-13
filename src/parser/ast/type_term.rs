use super::traits::ast_term::ASTTerm;
use super::traits::module_term_type::ModuleTermType;
use super::traits::module_usable_term::ModuleUsableTerm;
use super::traits::service_term_type::ServiceTermType;
use super::traits::service_usable_term::ServiceUsableTerm;
use crate::ir::ir_component::IRComponent;

impl ModuleUsableTerm for TypeTerm {
    fn get_type(&self) -> ModuleTermType {
        ModuleTermType::Type
    }
}

impl ServiceUsableTerm for TypeTerm {
    fn get_type(&self) -> ServiceTermType {
        ServiceTermType::Type
    }
}

impl ASTTerm for TypeTerm {
    fn generate_ir(&self) -> Box<IRComponent> {
        let fields = vec![
            ("name", self.name.generate_ir()),
            ("resolvesTo", self.resolved_type_name.generate_ir()),
        ];

        Box::new(IRComponent::new_object_from_vec(fields))
    }
}

pub struct TypeTerm {
    name: Box<dyn ASTTerm>,
    resolved_type_name: Box<dyn ASTTerm>,
}

impl TypeTerm {
    pub fn new_boxed(
        name: Box<dyn ASTTerm>,
        resolved_type_name: Box<dyn ASTTerm>,
    ) -> Box<TypeTerm> {
        Box::new(TypeTerm::new(name, resolved_type_name))
    }

    pub fn new(name: Box<dyn ASTTerm>, resolved_type_name: Box<dyn ASTTerm>) -> TypeTerm {
        TypeTerm {
            name,
            resolved_type_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ir::ir_component::IRComponent;

    use super::TypeTerm;
    use crate::parser::ast::name_term::NameTerm;

    use crate::parser::ast::traits::ast_term::ASTTerm;
    use crate::parser::ast::traits::service_term_type::ServiceTermType;
    use crate::parser::ast::traits::service_usable_term::ServiceUsableTerm;

    #[test]
    fn it_implements_service_usable_term_trait() {
        let term: Box<dyn ServiceUsableTerm> = TypeTerm::new_boxed(
            NameTerm::new_boxed("test".to_string()),
            NameTerm::new_boxed("TestType".to_string()),
        );

        assert_eq!(term.get_type(), ServiceTermType::Type);
    }

    #[test]
    fn it_generates_ir() {
        let type_term = TypeTerm::new_boxed(
            NameTerm::new_boxed("mock-name".to_string()),
            NameTerm::new_boxed("mock-type".to_string()),
        );
        let type_ir = type_term.generate_ir();

        let correct_ir_mock_type = vec![
            (
                "name",
                Box::new(IRComponent::new_string("mock-name".to_string())),
            ),
            (
                "resolvesTo",
                Box::new(IRComponent::new_string("mock-type".to_string())),
            ),
        ];
        let correct_ir = IRComponent::new_object_from_vec(correct_ir_mock_type);

        assert_eq!(type_ir, Box::new(correct_ir));
    }
}
