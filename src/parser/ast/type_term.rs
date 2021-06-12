use super::traits::ast_term::ASTTerm;
use super::traits::module_term_type::ModuleTermType;
use super::traits::module_usable_term::ModuleUsableTerm;
use super::traits::service_term_type::ServiceTermType;
use super::traits::service_usable_term::ServiceUsableTerm;
use crate::ir::ir_component::IRComponent;
use std::collections::HashMap;

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
        let mut fields = HashMap::<String, Box<IRComponent>>::new();
        fields.insert("name".to_string(), self.name.generate_ir());
        fields.insert(
            "resolvesTo".to_string(),
            self.resolved_type_name.generate_ir(),
        );

        Box::new(IRComponent::new_object(fields))
    }
}

#[cfg(test)]
mod tests {
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
}
