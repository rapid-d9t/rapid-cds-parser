use super::name_term::NameTerm;
use super::traits::ast_term::ASTTerm;
use super::traits::module_usable_term::ModuleUsableTerm;
use super::traits::service_usable_term::ServiceUsableTerm;

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

impl ModuleUsableTerm for TypeTerm {}

impl ServiceUsableTerm for TypeTerm {}

impl ASTTerm for TypeTerm {
    fn convert_to_json(&self) -> String {
        format!("{{ type: \"type\", name: \"{}\" }}", self.get_name())
    }
}
