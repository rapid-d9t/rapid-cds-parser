use super::name_term::NameTerm;
use super::param_term::ParamTerm;
use super::returns_term::ReturnsTerm;
use super::traits::ast_term::ASTTerm;
use super::traits::service_term_type::ServiceTermType;
use super::traits::service_usable_term::ServiceUsableTerm;
use crate::ir::function_ir::FunctionIR;
use crate::ir::ir_component::IRComponent;

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

impl ServiceUsableTerm for FunctionTerm {
    fn get_name(&self) -> String {
        self.name.get_value()
    }

    fn get_type(&self) -> ServiceTermType {
        ServiceTermType::Function
    }
}

impl ASTTerm for FunctionTerm {
    fn generate_ir(&self) -> Box<dyn IRComponent> {
        let params = self
            .params
            .iter()
            .map(|param| param.generate_ir())
            .collect();

        Box::new(FunctionIR::new(
            self.get_name(),
            params,
            self.returned_type.generate_ir(),
        ))
    }
}
