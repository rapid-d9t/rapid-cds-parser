use super::field_term::FieldTerm;
use super::name_term::NameTerm;
use super::traits::ast_term::ASTTerm;
use super::traits::module_usable_term::ModuleUsableTerm;
use super::traits::service_usable_term::ServiceUsableTerm;

pub struct EntityTerm {
    name: NameTerm,
    applied_aspects: Vec<NameTerm>,
    fields: Vec<FieldTerm>,
}

impl EntityTerm {
    pub fn new(
        name: NameTerm,
        applied_aspects: Vec<NameTerm>,
        fields: Vec<FieldTerm>,
    ) -> EntityTerm {
        EntityTerm {
            name,
            applied_aspects,
            fields,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.get_value()
    }
}

impl ModuleUsableTerm for EntityTerm {}

impl ServiceUsableTerm for EntityTerm {}

impl ASTTerm for EntityTerm {
    fn convert_to_json(&self) -> String {
        format!("{{ type: \"entity\", name: \"{}\" }}", self.get_name())
    }
}
