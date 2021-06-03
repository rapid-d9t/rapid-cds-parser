use super::traits::ast_term::ASTTerm;

pub struct NameTerm {
    value: String,
}

impl NameTerm {
    pub fn new(value: String) -> NameTerm {
        NameTerm { value }
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }
}
