use super::traits::ast_term::ASTTerm;
use crate::ir::ir_component::IRComponent;
use std::collections::HashMap;

pub struct FieldTerm {
    name: Box<dyn ASTTerm>,
    type_name: Box<dyn ASTTerm>,
}

impl FieldTerm {
    pub fn new_boxed(name: Box<dyn ASTTerm>, type_name: Box<dyn ASTTerm>) -> Box<FieldTerm> {
        Box::new(FieldTerm::new(name, type_name))
    }

    pub fn new(name: Box<dyn ASTTerm>, type_name: Box<dyn ASTTerm>) -> FieldTerm {
        FieldTerm { name, type_name }
    }
}

impl ASTTerm for FieldTerm {
    fn generate_ir(&self) -> Box<IRComponent> {
        let mut fields = HashMap::<String, Box<IRComponent>>::new();
        fields.insert("name".to_string(), self.name.generate_ir());
        fields.insert("type".to_string(), self.type_name.generate_ir());
        fields.insert(
            "hasDefault".to_string(),
            Box::new(IRComponent::new_bool(false)),
        );

        Box::new(IRComponent::new_object_from_map(fields))
    }
}

#[cfg(test)]
mod tests {
    use super::FieldTerm;
    use crate::ir::ir_component::IRComponent;
    use crate::parser::ast::name_term::NameTerm;
    use crate::parser::ast::traits::ast_term::ASTTerm;

    #[test]
    fn it_generates_ir() {
        let field_term = FieldTerm::new_boxed(
            NameTerm::new_boxed("mock-name".to_string()),
            NameTerm::new_boxed("mock-type".to_string()),
        );
        let field_ir = field_term.generate_ir();

        let correct_ir_mock_fields = vec![
            (
                "name",
                Box::new(IRComponent::new_string("mock-name".to_string())),
            ),
            (
                "type",
                Box::new(IRComponent::new_string("mock-type".to_string())),
            ),
            ("hasDefault", Box::new(IRComponent::new_bool(false))),
        ];
        let correct_ir = IRComponent::new_object_from_vec(correct_ir_mock_fields);

        assert_eq!(field_ir, Box::new(correct_ir));
    }
}
