use super::traits::ast_term::ASTTerm;
use super::traits::service_term_type::ServiceTermType;
use super::traits::service_usable_term::ServiceUsableTerm;
use crate::ir::ir_component::IRComponent;

pub struct FunctionTerm {
    name: Box<dyn ASTTerm>,
    params: Vec<Box<dyn ASTTerm>>,
    returned_type: Box<dyn ASTTerm>,
}

impl ServiceUsableTerm for FunctionTerm {
    fn get_type(&self) -> ServiceTermType {
        ServiceTermType::Function
    }
}

impl ASTTerm for FunctionTerm {
    fn generate_ir(&self) -> Box<IRComponent> {
        let fields = vec![
            ("name", self.name.generate_ir()),
            ("params", self.generate_params_ir()),
            ("output", self.returned_type.generate_ir()),
        ];

        Box::new(IRComponent::new_object_from_vec(fields))
    }
}

impl FunctionTerm {
    fn generate_params_ir(&self) -> Box<IRComponent> {
        let params = self
            .params
            .iter()
            .map(|param| param.generate_ir())
            .collect();

        Box::new(IRComponent::new_array(params))
    }

    pub fn new_boxed(
        name: Box<dyn ASTTerm>,
        params: Vec<Box<dyn ASTTerm>>,
        returned_type: Box<dyn ASTTerm>,
    ) -> Box<FunctionTerm> {
        Box::new(FunctionTerm::new(name, params, returned_type))
    }

    pub fn new(
        name: Box<dyn ASTTerm>,
        params: Vec<Box<dyn ASTTerm>>,
        returned_type: Box<dyn ASTTerm>,
    ) -> FunctionTerm {
        FunctionTerm {
            name,
            params,
            returned_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FunctionTerm;
    use crate::ir::ir_component::IRComponent;
    use crate::parser::ast::name_term::NameTerm;
    use crate::parser::ast::traits::ast_term::ASTTerm;
    use crate::parser::ast::traits::service_term_type::ServiceTermType;
    use crate::parser::ast::traits::service_usable_term::ServiceUsableTerm;

    #[test]
    fn it_implements_service_usable_term_trait() {
        let function_term = FunctionTerm::new_boxed(
            NameTerm::new_boxed("mock".to_string()),
            vec![],
            NameTerm::new_boxed("mock".to_string()),
        );

        assert_eq!(function_term.get_type(), ServiceTermType::Function);
    }

    #[test]
    fn with_empty_params_it_generates_ir() {
        let function_term = FunctionTerm::new_boxed(
            NameTerm::new_boxed("mock".to_string()),
            vec![],
            NameTerm::new_boxed("mock".to_string()),
        );
        let function_ir = function_term.generate_ir();

        let correct_ir_mock_fields = vec![
            (
                "name",
                Box::new(IRComponent::new_string("mock".to_string())),
            ),
            ("params", Box::new(IRComponent::new_array(vec![]))),
            (
                "output",
                Box::new(IRComponent::new_string("mock".to_string())),
            ),
        ];
        let correct_ir = IRComponent::new_object_from_vec(correct_ir_mock_fields);

        assert_eq!(function_ir, Box::new(correct_ir));
    }

    #[test]
    fn with_some_params_and_output_it_generates_ir() {
        let function_term = FunctionTerm::new_boxed(
            NameTerm::new_boxed("mock".to_string()),
            vec![NameTerm::new_boxed("mock".to_string())],
            NameTerm::new_boxed("mock".to_string()),
        );
        let function_ir = function_term.generate_ir();

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
            (
                "output",
                Box::new(IRComponent::new_string("mock".to_string())),
            ),
        ];
        let correct_ir = IRComponent::new_object_from_vec(correct_ir_mock_fields);

        assert_eq!(function_ir, Box::new(correct_ir));
    }
}
