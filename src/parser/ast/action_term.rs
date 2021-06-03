use super::name_term::NameTerm;
use super::param_term::ParamTerm;
use super::returns_term::ReturnsTerm;
use super::traits::ast_term::ASTTerm;
use super::traits::service_usable_term::ServiceUsableTerm;

pub struct ActionTerm {
    name: NameTerm,
    params: Vec<ParamTerm>,
    returned_type: Option<ReturnsTerm>,
}

impl ActionTerm {
    pub fn new(
        name: NameTerm,
        params: Vec<ParamTerm>,
        returned_type: Option<ReturnsTerm>,
    ) -> ActionTerm {
        ActionTerm {
            name,
            params,
            returned_type,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.get_value()
    }
}

impl ServiceUsableTerm for ActionTerm {}

impl ASTTerm for ActionTerm {
    fn convert_to_json(&self) -> String {
        format!("{{ type: \"action\", name: \"{}\" }}", self.get_name())
    }
}
