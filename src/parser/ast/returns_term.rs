use super::name_term::NameTerm;
use super::traits::ast_term::ASTTerm;
use crate::ir::ir_component::IRComponent;
use std::collections::HashMap;

pub struct ReturnsTerm {
    type_name: NameTerm,
    is_arrayed: bool,
}

impl ReturnsTerm {
    pub fn new(type_name: NameTerm) -> ReturnsTerm {
        ReturnsTerm {
            type_name,
            is_arrayed: false,
        }
    }

    pub fn new_arrayed(type_name: NameTerm) -> ReturnsTerm {
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
            Box::new(IRComponent::new_bool(false)),
        );

        Box::new(IRComponent::new_object(fields))
    }
}
