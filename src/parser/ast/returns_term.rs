use super::name_term::NameTerm;
use super::traits::ast_term::ASTTerm;

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

    pub fn get_type_name(&self) -> String {
        self.type_name.get_value()
    }
}
