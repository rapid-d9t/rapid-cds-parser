use super::traits::ast_term::ASTTerm;
use super::traits::service_term_type::ServiceTermType;
use super::traits::service_usable_term::ServiceUsableTerm;
use crate::ir::ir_component::IRComponent;
use std::collections::HashMap;

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
        match &self.returned_type {
            Some(returned_type) => {
                fields.insert("output".to_string(), returned_type.generate_ir());
                fields.insert(
                    "hasOutput".to_string(),
                    Box::new(IRComponent::new_bool(true)),
                );
            }
            None => {
                fields.insert(
                    "hasOutput".to_string(),
                    Box::new(IRComponent::new_bool(false)),
                );
            }
        }

        Box::new(IRComponent::new_object_from_map(fields))
    }
}

impl ActionTerm {
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
