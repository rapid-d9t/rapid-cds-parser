use super::name_term::NameTerm;
use super::param_term::ParamTerm;
use super::returns_term::ReturnsTerm;
use super::traits::ast_term::ASTTerm;
use super::traits::service_usable_term::ServiceUsableTerm;

pub struct FunctionTerm {
    name: NameTerm,
    params: Vec<ParamTerm>,
    returned_type: ReturnsTerm,
}

impl FunctionTerm {
    pub fn new(name: NameTerm, params: Vec<ParamTerm>, returned_type: ReturnsTerm) -> FunctionTerm {
        FunctionTerm {
            name,
            params,
            returned_type,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.get_value()
    }

    pub fn get_returned_type_name(&self) -> String {
        self.returned_type.get_type_name()
    }
}

impl ServiceUsableTerm for FunctionTerm {}

impl ASTTerm for FunctionTerm {
    fn convert_to_json(&self) -> String {
        format!("{{ type: \"function\", name: \"{}\" }}", self.get_name())
    }
}
