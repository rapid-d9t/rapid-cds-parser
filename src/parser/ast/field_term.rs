use super::traits::ast_term::ASTTerm;
use crate::ir::ir_component::IRComponent;

pub struct FieldTerm {
    name: Box<dyn ASTTerm>,
    type_name: Box<dyn ASTTerm>,
    type_arguments: Vec<Box<dyn ASTTerm>>,
}

impl ASTTerm for FieldTerm {
    fn generate_ir(&self) -> Box<IRComponent> {
        let mut fields = vec![
            ("name", self.name.generate_ir()),
            ("type", self.type_name.generate_ir()),
            ("hasDefault", Box::new(IRComponent::new_bool(false))),
        ];

        if self.type_arguments.len() > 0 {
            fields.push(("typeArguments", self.generate_type_arguments_ir()))
        }

        Box::new(IRComponent::new_object_from_vec(fields))
    }
}

impl FieldTerm {
    pub fn new_boxed(
        name: Box<dyn ASTTerm>,
        type_name: Box<dyn ASTTerm>,
        type_arguments: Option<Vec<Box<dyn ASTTerm>>>,
    ) -> Box<FieldTerm> {
        Box::new(FieldTerm::new(name, type_name, type_arguments))
    }

    pub fn new(
        name: Box<dyn ASTTerm>,
        type_name: Box<dyn ASTTerm>,
        type_arguments: Option<Vec<Box<dyn ASTTerm>>>,
    ) -> FieldTerm {
        let type_arguments = type_arguments.unwrap_or(Vec::new());
        FieldTerm {
            name,
            type_name,
            type_arguments,
        }
    }

    fn generate_type_arguments_ir(&self) -> Box<IRComponent> {
        Box::new(IRComponent::new_array(
            self.type_arguments
                .iter()
                .map(|param| param.generate_ir())
                .collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::FieldTerm;
    use crate::ir::ir_component::IRComponent;
    use crate::parser::ast::name_term::NameTerm;
    use crate::parser::ast::traits::ast_term::ASTTerm;
    use crate::parser::ast::type_argument_term::TypeArgumentTerm;

    #[test]
    fn it_generates_ir() {
        let field_term = FieldTerm::new_boxed(
            NameTerm::new_boxed("mock-name".to_string()),
            NameTerm::new_boxed("mock-type".to_string()),
            None,
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

    #[test]
    fn it_generates_ir_with_type_arguments() {
        let field_term = FieldTerm::new_boxed(
            NameTerm::new_boxed("mock-name".to_string()),
            NameTerm::new_boxed("mock-type".to_string()),
            Some(vec![
                TypeArgumentTerm::new_boxed("123".to_string()),
                TypeArgumentTerm::new_boxed("5467".to_string()),
            ]),
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
            (
                "typeArguments",
                Box::new(IRComponent::new_array(vec![
                    Box::new(IRComponent::new_number(123.)),
                    Box::new(IRComponent::new_number(5467.)),
                ])),
            ),
        ];

        let correct_ir = IRComponent::new_object_from_vec(correct_ir_mock_fields);

        assert_eq!(field_ir, Box::new(correct_ir));
    }
}
