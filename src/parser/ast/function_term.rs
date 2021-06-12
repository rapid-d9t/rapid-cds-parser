use super::traits::ast_term::ASTTerm;
use super::traits::service_term_type::ServiceTermType;
use super::traits::service_usable_term::ServiceUsableTerm;
use crate::ir::ir_component::IRComponent;
use std::collections::HashMap;

pub struct FunctionTerm {
    name: Box<dyn ASTTerm>,
    params: Vec<Box<dyn ASTTerm>>,
    returned_type: Box<dyn ASTTerm>,
}

impl FunctionTerm {
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

impl ServiceUsableTerm for FunctionTerm {
    fn get_type(&self) -> ServiceTermType {
        ServiceTermType::Function
    }
}

impl ASTTerm for FunctionTerm {
    fn generate_ir(&self) -> Box<IRComponent> {
        let params = self
            .params
            .iter()
            .map(|param| param.generate_ir())
            .collect();

        let mut fields = HashMap::<String, Box<IRComponent>>::new();
        fields.insert("name".to_string(), self.name.generate_ir());
        fields.insert(
            "params".to_string(),
            Box::new(IRComponent::new_array(params)),
        );
        fields.insert("output".to_string(), self.returned_type.generate_ir());

        Box::new(IRComponent::new_object(fields))
    }
}
