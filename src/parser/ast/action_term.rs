use super::name_term::NameTerm;
use super::param_term::ParamTerm;
use super::returns_term::ReturnsTerm;
use super::traits::ast_term::ASTTerm;
use super::traits::service_term_type::ServiceTermType;
use super::traits::service_usable_term::ServiceUsableTerm;
use crate::ir::action_ir::ActionIR;
use crate::ir::ir_component::IRComponent;

pub struct ActionTerm {
    name: NameTerm,
    params: Vec<ParamTerm>,
    returned_type: Option<ReturnsTerm>,
}

impl ServiceUsableTerm for ActionTerm {
    fn get_name(&self) -> String {
        self.name.get_value()
    }

    fn get_type(&self) -> ServiceTermType {
        ServiceTermType::Action
    }
}

impl ASTTerm for ActionTerm {
    fn generate_ir(&self) -> Box<dyn IRComponent> {
        let params = self
            .params
            .iter()
            .map(|param| param.generate_ir())
            .collect();

        Box::new(match &self.returned_type {
            Some(returned_type) => {
                ActionIR::new_with_output(self.get_name(), params, returned_type.generate_ir())
            }
            None => ActionIR::new(self.get_name(), params),
        })
    }
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
