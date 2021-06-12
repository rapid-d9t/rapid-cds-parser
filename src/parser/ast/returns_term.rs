use super::traits::ast_term::ASTTerm;
use crate::ir::ir_component::IRComponent;
use std::collections::HashMap;

pub struct ReturnsTerm {
    type_name: Box<dyn ASTTerm>,
    is_arrayed: bool,
}

impl ReturnsTerm {
    pub fn new_boxed(type_name: Box<dyn ASTTerm>) -> Box<ReturnsTerm> {
        Box::new(ReturnsTerm::new(type_name))
    }

    pub fn new(type_name: Box<dyn ASTTerm>) -> ReturnsTerm {
        ReturnsTerm {
            type_name,
            is_arrayed: false,
        }
    }

    pub fn new_arrayed_boxed(type_name: Box<dyn ASTTerm>) -> Box<ReturnsTerm> {
        Box::new(ReturnsTerm::new_arrayed(type_name))
    }

    pub fn new_arrayed(type_name: Box<dyn ASTTerm>) -> ReturnsTerm {
        ReturnsTerm {
            type_name,
            is_arrayed: true,
        }
    }
}

impl ASTTerm for ReturnsTerm {
    fn generate_ir(&self) -> Box<IRComponent> {
        let mut fields = HashMap::<String, Box<IRComponent>>::new();
        fields.insert("type".to_string(), self.type_name.generate_ir());
        fields.insert(
            "isArrayed".to_string(),
            Box::new(IRComponent::new_bool(self.is_arrayed)),
        );

        Box::new(IRComponent::new_object_from_map(fields))
    }
}
