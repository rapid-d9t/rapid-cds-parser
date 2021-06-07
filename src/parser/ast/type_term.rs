use super::name_term::NameTerm;
use super::traits::ast_term::ASTTerm;
use super::traits::module_term_type::ModuleTermType;
use super::traits::module_usable_term::ModuleUsableTerm;
use super::traits::service_term_type::ServiceTermType;
use super::traits::service_usable_term::ServiceUsableTerm;
use crate::ir::ir_component::IRComponent;
use crate::ir::type_ir::TypeIR;

pub struct TypeTerm {
    name: NameTerm,
    resolved_type_name: NameTerm,
}

impl TypeTerm {
    pub fn new(name: NameTerm, resolved_type_name: NameTerm) -> TypeTerm {
        TypeTerm {
            name,
            resolved_type_name,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.get_value()
    }

    pub fn get_resolved_type_name(&self) -> String {
        self.resolved_type_name.get_value()
    }
}

impl ModuleUsableTerm for TypeTerm {
    fn get_type(&self) -> ModuleTermType {
        ModuleTermType::Type
    }
}

impl ServiceUsableTerm for TypeTerm {
    fn get_name(&self) -> String {
        self.name.get_value()
    }

    fn get_type(&self) -> ServiceTermType {
        ServiceTermType::Type
    }
}

impl ASTTerm for TypeTerm {
    fn generate_ir(&self) -> Box<dyn IRComponent> {
        Box::new(TypeIR::new(self.get_name(), self.get_resolved_type_name()))
    }
}
