use super::traits::ast_term::ASTTerm;
use super::traits::service_term_type::ServiceTermType;
use super::traits::service_usable_term::ServiceUsableTerm;
use crate::ir::ir_component::IRComponent;

pub struct ActionTerm {
    name: Box<dyn ASTTerm>,
    params: Vec<Box<dyn ASTTerm>>,
    returned_type: Option<Box<dyn ASTTerm>>,
}

impl ServiceUsableTerm for ActionTerm {
    fn get_type(&self) -> ServiceTermType {
        ServiceTermType::Action
    }
}

impl ASTTerm for ActionTerm {
    fn generate_ir(&self) -> Box<IRComponent> {
        let mut fields = vec![
            ("name", self.name.generate_ir()),
            ("params", self.generate_params_ir()),
        ];

        self.add_output_field(&mut fields);

        Box::new(IRComponent::new_object_from_vec(fields))
    }
}

impl ActionTerm {
    fn generate_params_ir(&self) -> Box<IRComponent> {
        let params = self
            .params
            .iter()
            .map(|param| param.generate_ir())
            .collect();

        Box::new(IRComponent::new_array(params))
    }

    fn add_output_field(&self, fields: &mut Vec<(&str, Box<IRComponent>)>) {
        if let Some(returned_type) = &self.returned_type {
            fields.push(("output", returned_type.generate_ir()));
            fields.push(("hasOutput", Box::new(IRComponent::new_bool(true))));
        } else {
            fields.push(("hasOutput", Box::new(IRComponent::new_bool(false))));
        }
    }

    pub fn new_boxed(
        name: Box<dyn ASTTerm>,
        params: Vec<Box<dyn ASTTerm>>,
        returned_type: Option<Box<dyn ASTTerm>>,
    ) -> Box<ActionTerm> {
        Box::new(ActionTerm::new(name, params, returned_type))
    }

    pub fn new(
        name: Box<dyn ASTTerm>,
        params: Vec<Box<dyn ASTTerm>>,
        returned_type: Option<Box<dyn ASTTerm>>,
    ) -> ActionTerm {
        ActionTerm {
            name,
            params,
            returned_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ActionTerm;
    use crate::ir::ir_component::IRComponent;
    use crate::parser::ast::name_term::NameTerm;
    use crate::parser::ast::traits::ast_term::ASTTerm;
    use crate::parser::ast::traits::service_term_type::ServiceTermType;
    use crate::parser::ast::traits::service_usable_term::ServiceUsableTerm;

    #[test]
    fn it_implements_service_usable_term_trait() {
        let action_term =
            ActionTerm::new_boxed(NameTerm::new_boxed("mock".to_string()), vec![], None);

        assert_eq!(action_term.get_type(), ServiceTermType::Action);
    }

    #[test]
    fn with_empty_params_it_generates_ir() {
        let action_term =
            ActionTerm::new_boxed(NameTerm::new_boxed("mock".to_string()), vec![], None);
        let action_ir = action_term.generate_ir();

        let correct_ir_mock_fields = vec![
            (
                "name",
                Box::new(IRComponent::new_string("mock".to_string())),
            ),
            ("params", Box::new(IRComponent::new_array(vec![]))),
            ("hasOutput", Box::new(IRComponent::new_bool(false))),
        ];
        let correct_ir = IRComponent::new_object_from_vec(correct_ir_mock_fields);

        assert_eq!(action_ir, Box::new(correct_ir));
    }

    #[test]
    fn with_empty_params_and_output_it_generates_ir() {
        let action_term = ActionTerm::new_boxed(
            NameTerm::new_boxed("mock".to_string()),
            vec![],
            Some(NameTerm::new_boxed("mock".to_string())),
        );
        let action_ir = action_term.generate_ir();

        let correct_ir_mock_fields = vec![
            (
                "name",
                Box::new(IRComponent::new_string("mock".to_string())),
            ),
            ("params", Box::new(IRComponent::new_array(vec![]))),
            ("hasOutput", Box::new(IRComponent::new_bool(true))),
            (
                "output",
                Box::new(IRComponent::new_string("mock".to_string())),
            ),
        ];
        let correct_ir = IRComponent::new_object_from_vec(correct_ir_mock_fields);

        assert_eq!(action_ir, Box::new(correct_ir));
    }

    #[test]
    fn with_some_params_it_generates_ir() {
        let action_term = ActionTerm::new_boxed(
            NameTerm::new_boxed("mock".to_string()),
            vec![NameTerm::new_boxed("mock".to_string())],
            None,
        );
        let action_ir = action_term.generate_ir();

        let correct_ir_mock_fields = vec![
            (
                "name",
                Box::new(IRComponent::new_string("mock".to_string())),
            ),
            (
                "params",
                Box::new(IRComponent::new_array(vec![Box::new(
                    IRComponent::new_string("mock".to_string()),
                )])),
            ),
            ("hasOutput", Box::new(IRComponent::new_bool(false))),
        ];
        let correct_ir = IRComponent::new_object_from_vec(correct_ir_mock_fields);

        assert_eq!(action_ir, Box::new(correct_ir));
    }

    #[test]
    fn with_some_params_and_output_it_generates_ir() {
        let action_term = ActionTerm::new_boxed(
            NameTerm::new_boxed("mock".to_string()),
            vec![NameTerm::new_boxed("mock".to_string())],
            Some(NameTerm::new_boxed("mock".to_string())),
        );
        let action_ir = action_term.generate_ir();

        let correct_ir_mock_fields = vec![
            (
                "name",
                Box::new(IRComponent::new_string("mock".to_string())),
            ),
            (
                "params",
                Box::new(IRComponent::new_array(vec![Box::new(
                    IRComponent::new_string("mock".to_string()),
                )])),
            ),
            ("hasOutput", Box::new(IRComponent::new_bool(true))),
            (
                "output",
                Box::new(IRComponent::new_string("mock".to_string())),
            ),
        ];
        let correct_ir = IRComponent::new_object_from_vec(correct_ir_mock_fields);

        assert_eq!(action_ir, Box::new(correct_ir));
    }
}
